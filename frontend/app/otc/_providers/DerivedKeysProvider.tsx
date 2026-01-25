"use client";

import { createContext, useContext, ReactNode } from "react";
import { useDerivedKeys, UseDerivedKeysReturn } from "../_hooks/useDerivedKeys";

/**
 * Context value type - same as UseDerivedKeysReturn
 */
type DerivedKeysContextValue = UseDerivedKeysReturn;

const DerivedKeysContext = createContext<DerivedKeysContextValue | null>(null);

/**
 * Provider component that makes derived keys available throughout the app.
 * Must be placed inside SolanaProvider (requires wallet context).
 *
 * @example
 * ```tsx
 * // In layout.tsx
 * <SolanaProvider>
 *   <DerivedKeysProvider>
 *     {children}
 *   </DerivedKeysProvider>
 * </SolanaProvider>
 * ```
 */
export function DerivedKeysProvider({ children }: { children: ReactNode }) {
  const derivedKeysState = useDerivedKeys();

  return (
    <DerivedKeysContext.Provider value={derivedKeysState}>
      {children}
    </DerivedKeysContext.Provider>
  );
}

/**
 * Hook to access derived keys context.
 * Must be used within a DerivedKeysProvider.
 *
 * @throws Error if used outside of DerivedKeysProvider
 *
 * @example
 * ```tsx
 * function MyComponent() {
 *   const { derivedKeys, deriveKeysFromWallet, hasDerivedKeys } = useDerivedKeysContext();
 *   // ...
 * }
 * ```
 */
export function useDerivedKeysContext(): DerivedKeysContextValue {
  const context = useContext(DerivedKeysContext);
  if (!context) {
    throw new Error(
      "useDerivedKeysContext must be used within a DerivedKeysProvider"
    );
  }
  return context;
}
