// Token types
export const TOKENS = ["META", "ETH", "SOL", "USDC"] as const;
export type Token = (typeof TOKENS)[number];

export const PAIRS = [
  { base: "META", quote: "USDC", label: "META/USDC" },
  { base: "ETH", quote: "USDC", label: "ETH/USDC" },
  { base: "SOL", quote: "USDC", label: "SOL/USDC" },
] as const;

export type Pair = (typeof PAIRS)[number];

// Your Deals - deals created by user
export interface Deal {
  id: string;
  type: "buy" | "sell";
  pair: string;
  amount: number;
  price: number;
  total: number;
  status: "open" | "executed" | "expired";
  isPartial: boolean;
  allowPartial: boolean;
  expiresAt: number;
  createdAt: number;
  offerCount?: number;
}

// Open Market - other users' deals (no price shown)
export interface MarketDeal {
  id: string;
  type: "buy" | "sell";
  pair: string;
  expiresAt: number;
  createdAt: number;
  isPartial: boolean;
  // Mock data for deal details view
  size?: number;
  offerCount?: number;
}

// Your Offers - offers submitted by user
export interface Offer {
  id: string;
  pair: string;
  side: "buy" | "sell";
  amount: number;
  yourPrice: number;
  submittedAt: string;
  dealStatus: "open" | "executed" | "expired";
  offerStatus: "pending" | "passed" | "partial" | "failed";
}
