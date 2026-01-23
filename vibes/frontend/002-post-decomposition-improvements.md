# Post-Decomposition Improvements

**Date:** 2026-01-22
**Status:** Planning

---

## Current State After Decomposition

The OTC page has been decomposed from 1063 lines to 126 lines, with 14 extracted files.

| Component | Lines | Status |
|-----------|-------|--------|
| `page.tsx` | 126 | Clean orchestrator |
| `CreateDealForm.tsx` | 289 | **Largest - needs work** |
| `MakeOfferForm.tsx` | 154 | Has duplicated patterns |
| `DealDetails.tsx` | 99 | Minor improvements possible |
| `MarketTable.tsx` | 92 | Filter buttons hardcoded |
| `DealsTable.tsx` | 78 | Status rendering inline |
| `OffersTable.tsx` | 76 | Complex status logic inline |
| `FAQPanel.tsx` | 68 | Clean |
| `TabNavigation.tsx` | 58 | Clean |
| `Navbar.tsx` | 21 | Clean |
| `TokenIcon.tsx` | 30 | Clean |
| `FormInput.tsx` | 47 | Created but unused |

---

## Improvement Tasks

### Task 1: Extract TokenDropdown Component
**Priority:** High
**Impact:** ~100 lines removed from CreateDealForm
**Status:** [ ] Not started

`CreateDealForm` has two nearly identical token dropdowns (lines 111-148 and 167-204).

**Current duplication:**
```typescript
// Appears twice with minor differences (sellToken vs quoteToken)
<div className="relative" ref={sellDropdownRef}>
  <button onClick={() => setSellTokenDropdownOpen(!sellTokenDropdownOpen)}>
    <TokenIcon token={sellToken} />
    <span>{sellToken}</span>
    <ChevronDown />
  </button>
  {sellTokenDropdownOpen && (
    <div className="absolute ...">
      {TOKENS.filter(t => t !== quoteToken).map((token) => (...))}
    </div>
  )}
</div>
```

**Proposed interface:**
```typescript
interface TokenDropdownProps {
  selected: Token;
  onSelect: (token: Token) => void;
  exclude?: Token;  // Token to exclude from list
  disabled?: boolean;
}
```

**Files affected:**
- Create: `_components/TokenDropdown.tsx`
- Modify: `CreateDealForm.tsx`

---

### Task 2: Extract handleNumberInput to Utils
**Priority:** Medium
**Impact:** DRY, ~12 lines
**Status:** [ ] Not started

This function is duplicated in both form components:

```typescript
// In CreateDealForm (line 47-52) AND MakeOfferForm (line 30-35)
const handleNumberInput = (value: string, setter: (val: string) => void) => {
  const cleaned = value.replace(/,/g, "");
  if (cleaned === "" || /^\d*\.?\d*$/.test(cleaned)) {
    setter(cleaned);
  }
};
```

**Proposed location:** `_lib/format.ts`

```typescript
export const sanitizeNumberInput = (value: string): string | null => {
  const cleaned = value.replace(/,/g, "");
  if (cleaned === "" || /^\d*\.?\d*$/.test(cleaned)) {
    return cleaned;
  }
  return null;  // Invalid input
};
```

**Files affected:**
- Modify: `_lib/format.ts`
- Modify: `CreateDealForm.tsx`
- Modify: `MakeOfferForm.tsx`

---

### Task 3: Create StatusBadge Component
**Priority:** Medium
**Impact:** Unified status rendering, ~40 lines
**Status:** [ ] Not started

Four different inline status patterns exist:

1. **DealsTable** (lines 50-61): open/executed/expired with offer count
2. **MarketTable** (lines 67-72): offers count or "Open"
3. **OffersTable** (lines 50-61): pending/filled/partial/unfilled
4. **DealDetails** (lines 66-74): "has offers"/"open" badges

**Proposed variants:**
```typescript
type StatusVariant =
  | "open"
  | "executed"
  | "expired"
  | "pending"
  | "filled"
  | "partial"
  | "unfilled"
  | "has-offers";

interface StatusBadgeProps {
  variant: StatusVariant;
  children?: React.ReactNode;  // For custom text like "3 offers"
}
```

**Files affected:**
- Create: `_components/StatusBadge.tsx`
- Modify: `DealsTable.tsx`
- Modify: `MarketTable.tsx`
- Modify: `OffersTable.tsx`
- Modify: `DealDetails.tsx`

---

### Task 4: Use getPairFromLabel Consistently
**Priority:** Low
**Impact:** Consistency, ~8 lines
**Status:** [ ] Not started

`deal.pair.split("/")` appears in multiple places, but we already have `getPairFromLabel` in format.ts.

**Current inconsistency:**
```typescript
// In tables (manual split)
const [base, quote] = deal.pair.split("/");

// In MakeOfferForm (uses utility)
const { base, quote } = getPairFromLabel(deal.pair);
```

**Files affected:**
- Modify: `DealsTable.tsx`
- Modify: `MarketTable.tsx`
- Modify: `OffersTable.tsx`

---

### Task 5: Extract Spinner Component
**Priority:** Low
**Impact:** DRY, minor
**Status:** [ ] Not started

Same spinner SVG appears in:
- `CreateDealForm.tsx` (lines 278-281)
- `MakeOfferForm.tsx` (lines 143-146)

**Proposed component:**
```typescript
export const Spinner = ({ className = "h-4 w-4" }: { className?: string }) => (
  <svg className={`animate-spin ${className}`} ...>...</svg>
);
```

**Files affected:**
- Create: `_components/Spinner.tsx`
- Modify: `CreateDealForm.tsx`
- Modify: `MakeOfferForm.tsx`

---

### Task 6: Add Time Constants
**Priority:** Low
**Impact:** Readability
**Status:** [ ] Not started

Magic numbers in the codebase:
- `3600000` (1 hour in ms) in CreateDealForm line 81
- `7200000` (2 hours in ms) in format.ts `isUrgent` function

**Proposed constants in `_lib/constants.ts`:**
```typescript
export const MS_PER_HOUR = 3600000;
export const MS_PER_MINUTE = 60000;
export const URGENT_THRESHOLD_MS = 2 * MS_PER_HOUR;
```

---

### Task 7: Use FormInput Component
**Priority:** Low
**Impact:** Consistency
**Status:** [ ] Not started

`FormInput.tsx` was created during decomposition but never integrated.

Input pattern appears in:
- `CreateDealForm.tsx` (3 inputs + 1 readonly)
- `MakeOfferForm.tsx` (2 inputs + 1 readonly)

Could reduce boilerplate if we refine the FormInput interface.

---

## Execution Order

1. [ ] **TokenDropdown** - Biggest impact, reduces CreateDealForm significantly
2. [ ] **handleNumberInput** - Quick win while in the form files
3. [ ] **StatusBadge** - Unifies status across all tables
4. [ ] **getPairFromLabel** - Quick consistency fix
5. [ ] **Spinner** - Minor DRY improvement
6. [ ] **Time constants** - Minor readability improvement
7. [ ] **FormInput integration** - Optional, depends on needs

---

## Notes

- Each task should be done incrementally with verification after each step
- Run `yarn dev` and test `/otc` page after each change
- Keep components focused - don't over-abstract
