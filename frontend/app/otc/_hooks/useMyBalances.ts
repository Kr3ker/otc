"use client";

import { useState, useEffect, useCallback, useMemo } from "react";
import { useSupabase } from "../_providers/SupabaseProvider";
import { useDerivedKeysContext } from "../_providers/DerivedKeysProvider";
import { useMxePublicKey } from "../_providers/OtcProvider";
import {
  createDecryptionCipher,
  decryptBalanceData,
  bytesToHex,
} from "../_lib/decryption";

export interface Balance {
  /** Balance account address (base58) */
  address: string;
  /** Controller pubkey (base58) */
  controller: string;
  /** Token mint address (base58) */
  mint: string;
  /** Available balance amount (raw, not adjusted for decimals) */
  amount: bigint;
  /** Amount committed to open deals/offers */
  committedAmount: bigint;
}

interface UseMyBalancesReturn {
  balances: Balance[];
  isLoading: boolean;
  error: string | null;
  refetch: () => void;
  /** Get balance for a specific mint, returns undefined if not found */
  getBalance: (mint: string) => Balance | undefined;
}

/**
 * Fetches balances where encryption_key matches user's public key, then decrypts.
 * Requires derived keys and MXE public key to be available.
 */
export function useMyBalances(): UseMyBalancesReturn {
  const supabase = useSupabase();
  const { derivedKeys, hasDerivedKeys } = useDerivedKeysContext();
  const mxePublicKey = useMxePublicKey();

  const [balances, setBalances] = useState<Balance[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // User's encryption public key as hex for database comparison
  const userPubKeyHex = useMemo(() => {
    if (!hasDerivedKeys || !derivedKeys) return null;
    return "\\x" + bytesToHex(derivedKeys.encryption.publicKey);
  }, [derivedKeys, hasDerivedKeys]);

  const fetchBalances = useCallback(async () => {
    if (!userPubKeyHex || !mxePublicKey || !derivedKeys) {
      setBalances([]);
      setIsLoading(false);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      // Fetch all balances where encryption_key matches user's pubkey
      const { data, error: queryError } = await supabase
        .from("balances")
        .select("*")
        .eq("encryption_key", userPubKeyHex);

      if (queryError) throw queryError;

      // Create cipher for decryption
      const cipher = createDecryptionCipher(
        derivedKeys.encryption.privateKey,
        mxePublicKey
      );

      // Decrypt each balance
      const decrypted: Balance[] = (data ?? []).map((row) => {
        const { amount, committedAmount } = decryptBalanceData(
          row.ciphertexts,
          row.nonce,
          cipher
        );

        return {
          address: row.address,
          controller: row.controller,
          mint: row.mint,
          amount,
          committedAmount,
        };
      });

      setBalances(decrypted);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to fetch balances");
    } finally {
      setIsLoading(false);
    }
  }, [supabase, userPubKeyHex, mxePublicKey, derivedKeys]);

  useEffect(() => {
    fetchBalances();
  }, [fetchBalances]);

  // Realtime subscription for balance changes
  useEffect(() => {
    if (!userPubKeyHex) return;

    const channel = supabase
      .channel("my-balances-changes")
      .on(
        "postgres_changes",
        { event: "*", schema: "public", table: "balances" },
        () => {
          fetchBalances();
        }
      )
      .subscribe();

    return () => {
      supabase.removeChannel(channel);
    };
  }, [supabase, fetchBalances, userPubKeyHex]);

  // Helper to get balance for a specific mint
  const getBalance = useCallback(
    (mint: string): Balance | undefined => {
      return balances.find((b) => b.mint === mint);
    },
    [balances]
  );

  return { balances, isLoading, error, refetch: fetchBalances, getBalance };
}
