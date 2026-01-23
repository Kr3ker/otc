import type { Deal, MarketDeal, Offer } from "./types";

// Mock data
export const MOCK_DEALS: Deal[] = [
  { id: "d1", type: "buy", pair: "META/USDC", amount: 4444, price: 444, total: 1973136, status: "open", isPartial: false, allowPartial: true, expiresAt: Date.now() + 83640000, createdAt: Date.now(), offerCount: 0 },
  { id: "d2", type: "sell", pair: "ETH/USDC", amount: 10, price: 3200, total: 32000, status: "open", isPartial: true, allowPartial: true, expiresAt: Date.now() + 20520000, createdAt: Date.now() - 3600000, offerCount: 3 },
  { id: "d3", type: "buy", pair: "META/USDC", amount: 1000, price: 450, total: 450000, status: "executed", isPartial: true, allowPartial: false, expiresAt: 0, createdAt: Date.now() - 86400000, offerCount: 2 },
];

export const MOCK_MARKET_DEALS: MarketDeal[] = [
  { id: "mkt001", type: "buy", pair: "META/USDC", expiresAt: Date.now() + 9240000, createdAt: Date.now() - 14760000, isPartial: true, size: 5000, offerCount: 3 },
  { id: "mkt002", type: "sell", pair: "META/USDC", expiresAt: Date.now() + 51720000, createdAt: Date.now() - 34680000, isPartial: false, size: 2500, offerCount: 0 },
  { id: "mkt003", type: "buy", pair: "ETH/USDC", expiresAt: Date.now() + 22140000, createdAt: Date.now() - 64260000, isPartial: true, size: 15, offerCount: 2 },
  { id: "mkt004", type: "sell", pair: "ETH/USDC", expiresAt: Date.now() + 3900000, createdAt: Date.now() - 82500000, isPartial: true, size: 8, offerCount: 4 },
  { id: "mkt005", type: "buy", pair: "SOL/USDC", expiresAt: Date.now() + 67200000, createdAt: Date.now() - 19200000, isPartial: false, size: 100, offerCount: 0 },
];

export const MOCK_OFFERS: Offer[] = [
  { id: "off001", pair: "META/USDC", side: "sell", amount: 10, yourPrice: 442, submittedAt: "2h ago", dealStatus: "open", offerStatus: "pending" },
  { id: "off002", pair: "ETH/USDC", side: "sell", amount: 2, yourPrice: 3200, submittedAt: "5h ago", dealStatus: "executed", offerStatus: "passed" },
  { id: "off003", pair: "META/USDC", side: "buy", amount: 25, yourPrice: 448, submittedAt: "1d ago", dealStatus: "expired", offerStatus: "failed" },
  { id: "off004", pair: "SOL/USDC", side: "sell", amount: 50, yourPrice: 185, submittedAt: "3h ago", dealStatus: "open", offerStatus: "pending" },
  { id: "off005", pair: "ETH/USDC", side: "buy", amount: 1, yourPrice: 3150, submittedAt: "6h ago", dealStatus: "executed", offerStatus: "partial" },
];

// FAQ data for negotiation panel
export const FAQ_ITEMS = [
  {
    q: "What is an OTC RFQ?",
    a: "OTC RFQ (Request for Quote) lets you request private quotes from market makers for large trades without exposing your order to public markets."
  },
  {
    q: "How does private price discovery work?",
    a: "Your order details are encrypted. Market makers submit sealed bids that only you can see, preventing front-running and information leakage."
  },
  {
    q: "How is confidentiality preserved?",
    a: "All trade parameters are encrypted using Arcium's MPC network. Neither party sees the other's limits until a match is confirmed."
  },
  {
    q: "What happens after both sides agree?",
    a: "Once prices match, the trade executes atomically on-chain. Funds are swapped directly between wallets with no counterparty risk."
  }
];
