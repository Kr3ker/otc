# Mint-First Refactor Plan

> **STATUS: RESOLVED** - Completed 2026-01-23

---

## Problem

The frontend currently uses token **symbols** (e.g., "META", "ETH") as identifiers in many places. This is problematic because:

1. **Symbols are not unique** - Multiple tokens can have the same symbol
2. **Symbols can change** - Token metadata can be updated
3. **Symbol collisions** - Adding new tokens risks conflicts
4. **Type safety** - The `Token` type is a string union derived from hardcoded symbols

The **mint address** is the only true unique identifier for a Solana token.

---

## Current Architecture (Symbol-Keyed)

### Files Using Symbols as Keys

| File | Usage | Problem |
|------|-------|---------|
| `_lib/types.ts` | `TOKENS = ["META", "ETH", "SOL", "USDC"]` | Hardcoded symbol list |
| `_lib/types.ts` | `type Token = "META" \| "ETH" \| ...` | Type derived from symbols |
| `_components/TokenDropdown.tsx` | `onSelect: (token: Token) => void` | Passes symbols around |
| `_components/TokenIcon.tsx` | `Record<Token, JSX.Element>` | Icons keyed by symbol |
| `_components/CreateDealForm.tsx` | `useState<Token>("META")` | Form state uses symbols |
| `page.tsx` | `pairFilter === "META"` | Filters by symbol |
| `_components/MarketTable.tsx` | `["all", "META", "ETH", "SOL"]` | Hardcoded filter buttons |

### Data Flow (Current)

```
User selects "ETH" in dropdown
       ↓
CreateDealForm state: sellToken = "ETH" (symbol)
       ↓
On submit: getMintFromSymbol("ETH") → mint address
       ↓
Deal object: { baseMint: "7vfC...", quoteMint: "EPjF..." }
```

The conversion happens at the last moment, which is fragile.

---

## Target Architecture (Mint-Keyed)

### Principles

1. **Mints are the source of truth** - All internal state uses mint addresses
2. **Symbols are display-only** - Derive symbols from mints when rendering
3. **Single token registry** - `tokens.ts` is the only place defining supported tokens
4. **Type safety** - Create branded types for mints

### New Data Flow

```
User selects "ETH" in dropdown
       ↓
Dropdown calls onSelect(MINTS.ETH) → passes mint address
       ↓
CreateDealForm state: sellMint = "7vfC..." (mint)
       ↓
On submit: { baseMint: sellMint, quoteMint: quoteMint }
       ↓
Display: getTokenSymbol(baseMint) → "ETH"
```

---

## Implementation Plan

### Phase 1: Enhance Token Registry (`_lib/tokens.ts`)

**Current:**
```typescript
export const TOKEN_REGISTRY: Record<string, TokenInfo> = { ... }
```

**Target:**
```typescript
// Supported token mints (the source of truth)
export const SUPPORTED_MINTS = [
  "So11111111111111111111111111111111111111112",  // SOL
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
  "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs", // ETH
  "META111111111111111111111111111111111111111",   // META (mock)
] as const;

export type SupportedMint = (typeof SUPPORTED_MINTS)[number];

// Named exports for common mints (convenience)
export const MINTS = {
  SOL: "So11111111111111111111111111111111111111112",
  USDC: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  ETH: "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs",
  META: "META111111111111111111111111111111111111111",
} as const;

// Registry stays the same but we add a helper
export function isSupportedMint(mint: string): mint is SupportedMint {
  return SUPPORTED_MINTS.includes(mint as SupportedMint);
}
```

### Phase 2: Update Types (`_lib/types.ts`)

**Remove:**
```typescript
export const TOKENS = ["META", "ETH", "SOL", "USDC"] as const;
export type Token = (typeof TOKENS)[number];
```

**These are no longer needed** - we use `SupportedMint` from tokens.ts instead.

### Phase 3: Refactor TokenDropdown

**Current:**
```typescript
interface TokenDropdownProps {
  selected: Token;                    // symbol
  onSelect: (token: Token) => void;   // passes symbol
  exclude?: Token;                    // symbol
}
```

**Target:**
```typescript
interface TokenDropdownProps {
  selected: string;                   // mint address
  onSelect: (mint: string) => void;   // passes mint
  exclude?: string;                   // mint address
}

// Inside component:
{SUPPORTED_MINTS.filter((m) => m !== exclude).map((mint) => (
  <button
    key={mint}
    onClick={() => onSelect(mint)}
  >
    <TokenIcon mint={mint} />
    {getTokenSymbol(mint)}
  </button>
))}
```

### Phase 4: Refactor TokenIcon

**Current:**
```typescript
const icons: Record<Token, JSX.Element> = {
  META: <svg>...</svg>,
  ETH: <svg>...</svg>,
  ...
};
return icons[token];
```

**Target:**
```typescript
interface TokenIconProps {
  mint: string;
  className?: string;
}

// Icons keyed by mint address
const icons: Record<string, JSX.Element> = {
  [MINTS.META]: <svg>...</svg>,
  [MINTS.ETH]: <svg>...</svg>,
  [MINTS.SOL]: <svg>...</svg>,
  [MINTS.USDC]: <svg>...</svg>,
};

export const TokenIcon = ({ mint, className }: TokenIconProps) => {
  const icon = icons[mint];
  if (!icon) {
    // Fallback for unknown tokens
    return <DefaultTokenIcon className={className} />;
  }
  return icon;
};
```

### Phase 5: Refactor CreateDealForm

**Current:**
```typescript
const [sellToken, setSellToken] = useState<Token>("META");
const [quoteToken, setQuoteToken] = useState<Token>("USDC");
// ...
baseMint: getMintFromSymbol(sellToken)!,
```

**Target:**
```typescript
const [sellMint, setSellMint] = useState(MINTS.META);
const [quoteMint, setQuoteMint] = useState(MINTS.USDC);
// ...
baseMint: sellMint,
quoteMint: quoteMint,

// For display:
Price per {getTokenSymbol(sellMint)}
```

### Phase 6: Refactor Market Filters

**Current (page.tsx):**
```typescript
const [pairFilter, setPairFilter] = useState<string>("all");
// ...
marketDeals.filter((d) => getTokenSymbol(d.baseMint) === pairFilter);
```

**Current (MarketTable.tsx):**
```typescript
{["all", "META", "ETH", "SOL"].map((f) => ...)}
```

**Target (page.tsx):**
```typescript
const [baseMintFilter, setBaseMintFilter] = useState<string | null>(null);
// ...
marketDeals.filter((d) => baseMintFilter === null || d.baseMint === baseMintFilter);
```

**Target (MarketTable.tsx):**
```typescript
// Derive available base mints from deals
const availableBaseMints = [...new Set(deals.map(d => d.baseMint))];

{[null, ...availableBaseMints].map((mint) => (
  <button
    key={mint ?? "all"}
    onClick={() => onBaseMintFilterChange(mint)}
  >
    {mint === null ? "All Pairs" : getTokenSymbol(mint)}
  </button>
))}
```

---

## File-by-File Changes

### `_lib/tokens.ts`
- Add `SUPPORTED_MINTS` array
- Add `SupportedMint` type
- Add `MINTS` named constant object
- Add `isSupportedMint()` helper

### `_lib/types.ts`
- Remove `TOKENS` constant
- Remove `Token` type
- Keep interface definitions (Deal, MarketDeal, Offer unchanged)

### `_components/TokenDropdown.tsx`
- Change props from `Token` to `string` (mint)
- Import `SUPPORTED_MINTS`, `getTokenSymbol` from tokens.ts
- Update render to use mint keys

### `_components/TokenIcon.tsx`
- Change prop from `token: Token` to `mint: string`
- Key icons by mint address using `MINTS` constants
- Add fallback for unknown mints

### `_components/CreateDealForm.tsx`
- Change state from symbols to mints
- Remove `getMintFromSymbol` usage (no longer needed)
- Update display labels to use `getTokenSymbol()`

### `_components/MarketTable.tsx`
- Change filter from symbol to mint
- Update button rendering

### `page.tsx`
- Change `pairFilter` to `baseMintFilter`
- Update filter logic

---

## Migration Checklist

- [ ] Phase 1: Enhance `tokens.ts` with `SUPPORTED_MINTS`, `MINTS`, `SupportedMint`
- [ ] Phase 2: Remove `TOKENS` and `Token` from `types.ts`
- [ ] Phase 3: Refactor `TokenDropdown.tsx`
- [ ] Phase 4: Refactor `TokenIcon.tsx`
- [ ] Phase 5: Refactor `CreateDealForm.tsx`
- [ ] Phase 6: Refactor `MarketTable.tsx` filter buttons
- [ ] Phase 6: Refactor `page.tsx` filter state
- [ ] Verify: `yarn workspace frontend build`
- [ ] Verify: `yarn lint`
- [ ] Verify: Manual testing of all flows

---

## Benefits After Refactor

1. **True uniqueness** - Mint addresses are guaranteed unique
2. **Future-proof** - Adding tokens only requires updating `tokens.ts`
3. **Type safety** - `SupportedMint` type ensures only valid mints
4. **Simpler data flow** - No symbol↔mint conversions needed
5. **Extensible** - Easy to add token logos via URL, fetch metadata from chain, etc.

---

## Notes

- Keep `getMintFromSymbol()` temporarily for any edge cases
- Mock token META uses placeholder mint `META111...` - this will be replaced with real token when we integrate
- Consider adding token logos as URLs in `TokenInfo` for future use
