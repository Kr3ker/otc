# URL Routing with Search Params

**Date:** 2026-01-22
**Status:** Planning

---

## Goal

Add URL-based routing to the OTC page so that:
1. Users can share links to specific deals
2. Browser back/forward navigation works naturally
3. Live data connections persist across view changes

---

## Chosen Approach: URL Search Params

**Rationale:** The OTC page will have live data (WebSockets/subscriptions). Search params keep the main component mounted, so connections persist across view changes without reconnection overhead.

**URL Structure:**
```
/otc                        → Renders market (no redirect, preserves back button)
/otc?view=market            → Market table
/otc?view=deals             → Your deals table
/otc?view=offers            → Your offers table
/otc?view=market&deal=abc   → Deal details (shareable!)
```

Note: `/otc` without params renders market view but doesn't redirect (to avoid trapping back button). Any subsequent navigation adds explicit `view` param.

---

## Current State

`page.tsx` manages view state with `useState`:
```typescript
const [activeTab, setActiveTab] = useState<TabId>("market");
const [selectedMarketDeal, setSelectedMarketDeal] = useState<MarketDeal | null>(null);
```

Navigation happens via state setters:
- `setActiveTab("deals")` - switches tabs
- `setSelectedMarketDeal(deal)` - opens deal details
- `setSelectedMarketDeal(null)` - closes deal details

---

## Implementation Tasks

### Task 1: Create useUrlState Hook
**Priority:** High
**Files:** Create `_hooks/useUrlState.ts`

Custom hook that syncs URL search params with component state.

```typescript
interface UrlState {
  view: TabId;
  dealId: string | null;
}

interface UseUrlStateReturn {
  state: UrlState;
  setView: (view: TabId) => void;
  setDealId: (dealId: string | null) => void;
  navigateToDeal: (dealId: string) => void;
  navigateBack: () => void;
}

export function useUrlState(): UseUrlStateReturn {
  const searchParams = useSearchParams();
  const router = useRouter();
  const pathname = usePathname();

  const state: UrlState = {
    view: (searchParams.get("view") as TabId) || "market",
    dealId: searchParams.get("deal"),
  };

  const updateParams = (updates: Partial<UrlState>) => {
    const params = new URLSearchParams(searchParams.toString());

    if (updates.view !== undefined) {
      params.set("view", updates.view); // Always explicit
    }

    if (updates.dealId !== undefined) {
      if (updates.dealId === null) {
        params.delete("deal");
      } else {
        params.set("deal", updates.dealId);
      }
    }

    router.push(`${pathname}?${params.toString()}`);
  };

  return {
    state,
    setView: (view) => updateParams({ view, dealId: null }),
    setDealId: (dealId) => updateParams({ dealId }),
    navigateToDeal: (dealId) => updateParams({ view: "market", dealId }),
    navigateBack: () => updateParams({ dealId: null }),
  };
}
```

**Key decisions:**
- `view` param is added on any navigation (but bare `/otc` isn't redirected)
- `deal` param implies market view context
- `router.push` creates browser history entries (back button works)

---

### Task 2: Wrap Page in Suspense
**Priority:** High
**Files:** Modify `page.tsx`

`useSearchParams()` requires a Suspense boundary in Next.js App Router.

**Option A:** Wrap in page.tsx
```typescript
import { Suspense } from "react";

export default function OTCPage() {
  return (
    <Suspense fallback={<OTCPageSkeleton />}>
      <OTCPageContent />
    </Suspense>
  );
}

function OTCPageContent() {
  // ... existing component using useUrlState
}
```

**Option B:** Create loading.tsx
```typescript
// app/otc/loading.tsx
export default function Loading() {
  return <OTCPageSkeleton />;
}
```

**Recommendation:** Option A - keeps it self-contained and explicit.

---

### Task 3: Replace useState with useUrlState
**Priority:** High
**Files:** Modify `page.tsx`

Replace current state management:

**Before:**
```typescript
const [activeTab, setActiveTab] = useState<TabId>("market");
const [selectedMarketDeal, setSelectedMarketDeal] = useState<MarketDeal | null>(null);

const handleMarketDealClick = (deal: MarketDeal) => {
  setSelectedMarketDeal(deal);
};

const handleCollapse = () => {
  setSelectedMarketDeal(null);
};
```

**After:**
```typescript
const { state, setView, navigateToDeal, navigateBack } = useUrlState();

// Derive selected deal from URL + data
const selectedMarketDeal = state.dealId
  ? marketDeals.find(d => d.id === state.dealId) ?? null
  : null;

const handleMarketDealClick = (deal: MarketDeal) => {
  navigateToDeal(deal.id);
};

const handleCollapse = () => {
  navigateBack();
};
```

---

### Task 4: Update TabNavigation
**Priority:** Medium
**Files:** Modify `TabNavigation.tsx`, `page.tsx`

Change tab navigation to use URL-based navigation.

**Current:**
```typescript
<TabNavigation activeTab={activeTab} onTabChange={setActiveTab} />
```

**After:**
```typescript
<TabNavigation activeTab={state.view} onTabChange={setView} />
```

No changes needed to TabNavigation component itself - it just receives different handlers.

---

### Task 5: Update handleOfferPlaced and handleDealCreated
**Priority:** Medium
**Files:** Modify `page.tsx`

These handlers change tabs after actions:

**Before:**
```typescript
const handleDealCreated = (newDeal: Deal) => {
  setDeals((prev) => [newDeal, ...prev]);
  setActiveTab("deals");
};

const handleOfferPlaced = () => {
  setSelectedMarketDeal(null);
  setActiveTab("offers");
};
```

**After:**
```typescript
const handleDealCreated = (newDeal: Deal) => {
  setDeals((prev) => [newDeal, ...prev]);
  setView("deals");
};

const handleOfferPlaced = () => {
  setView("offers");  // This also clears dealId
};
```

---

### Task 6: Handle Invalid Deal IDs
**Priority:** Medium
**Files:** Modify `page.tsx`, create `_components/DealNotFound.tsx`

If someone visits `/otc?deal=nonexistent`, show a "Deal not found" UI.

```typescript
const selectedMarketDeal = state.dealId
  ? marketDeals.find(d => d.id === state.dealId) ?? null
  : null;

const dealNotFound = state.dealId && !selectedMarketDeal;
```

In the render:
```tsx
{dealNotFound ? (
  <DealNotFound onBack={navigateBack} />
) : selectedMarketDeal ? (
  <DealDetails deal={selectedMarketDeal} onBack={navigateBack} />
) : (
  // ... table view
)}
```

**DealNotFound component:**
```tsx
interface DealNotFoundProps {
  onBack: () => void;
}

export function DealNotFound({ onBack }: DealNotFoundProps) {
  return (
    <div className="p-8 text-center">
      <p className="text-muted-foreground mb-4">Deal not found</p>
      <button onClick={onBack} className="text-primary hover:underline">
        ← Back to market
      </button>
    </div>
  );
}
```

---

### Task 7: Add URL Persistence for Filters (Deferred)
**Priority:** Low
**Status:** Not implementing now

Pair filter will remain as local state. Can revisit later if shareable filtered views become needed.

---

## File Changes Summary

| File | Action | Description |
|------|--------|-------------|
| `_hooks/useUrlState.ts` | Create | URL state management hook |
| `_components/DealNotFound.tsx` | Create | "Deal not found" UI component |
| `page.tsx` | Modify | Replace useState with useUrlState, add Suspense |
| `TabNavigation.tsx` | None | Works as-is with new handlers |
| `types.ts` | None | Already has `id` field on all types |

---

## Testing Checklist

- [ ] Direct URL `/otc` loads market view
- [ ] Direct URL `/otc?view=deals` loads deals tab
- [ ] Direct URL `/otc?view=offers` loads offers tab
- [ ] Direct URL `/otc?deal=xxx` loads deal details
- [ ] Clicking deal row updates URL to `?deal=xxx`
- [ ] Back button returns to previous view
- [ ] Forward button works after going back
- [ ] Tab clicks update URL
- [ ] Creating deal navigates to deals tab (URL updates)
- [ ] Placing offer navigates to offers tab (URL updates)
- [ ] Invalid deal ID handled gracefully
- [ ] Refresh page preserves current view
- [ ] Copy/paste URL in new tab loads correct view

---

## Future Considerations

1. **Deep linking to specific deals from external sources** — Works out of the box with this approach

2. **Analytics** — URL changes can be tracked for page view analytics

3. **Live data integration** — When adding WebSockets, the single-component architecture makes subscription management straightforward

4. **Filter persistence** — Can extend URL state to include pair filter if shareable filtered views become needed

---

## Decisions

1. **Invalid deal IDs** → Show "Deal not found" UI with button to return to market
2. **URL explicitness** → Always include `view` param on navigation (but don't redirect bare `/otc` to preserve back button)
3. **Filters in URL** → No, keep pair filter as local state for now

---

## Notes

- Using `router.push()` creates history entries; `router.replace()` would not
- Next.js 16+ requires Suspense boundary for useSearchParams
- This approach keeps the door open for adding real-time data without architectural changes
