# OTC Comparison Component - Handoff Document

## Overview
Create a two-column comparison component that contrasts "Traditional OTC" vs "Veil OTC" trading. Use your own styling conventions and best practices.

## Reference Implementation

```jsx
import { X, Check, Shield, Users } from 'lucide-react';

function OTCComparison() {
  return (
    <section>
      {/* Optional Header */}
      <header>
        <h2>The OTC desk without third parties</h2>
        <p>Trustless and self-custodial trading</p>
      </header>

      <div className="grid-two-columns">
        {/* Left Card - Traditional OTC */}
        <div className="card card--muted">
          <div className="card__header">
            <div className="icon-wrapper icon-wrapper--negative">
              <Users />
            </div>
            <h3>Traditional OTC</h3>
          </div>
          
          <ul className="card__list">
            <li>
              <X className="icon icon--negative" />
              <span>Third party holds your funds</span>
            </li>
            <li>
              <X className="icon icon--negative" />
              <span>Requires trust in intermediaries</span>
            </li>
            <li>
              <X className="icon icon--negative" />
              <span>Risk of information disclosure</span>
            </li>
          </ul>
        </div>

        {/* Right Card - Veil OTC (Highlighted) */}
        <div className="card card--highlighted">
          <div className="card__header">
            <div className="icon-wrapper icon-wrapper--positive">
              <Shield />
            </div>
            <h3>Veil OTC</h3>
          </div>
          
          <ul className="card__list">
            <li>
              <Check className="icon icon--positive" />
              <span>Full control of your funds</span>
            </li>
            <li>
              <Check className="icon icon--positive" />
              <span>Trustless on-chain trading</span>
            </li>
            <li>
              <Check className="icon icon--positive" />
              <span>Encrypted, private execution</span>
            </li>
          </ul>
        </div>
      </div>
    </section>
  );
}
```

## Icons Used
From `lucide-react` (or equivalent icon library):

| Icon | Import | Usage |
|------|--------|-------|
| Users | `import { Users } from 'lucide-react'` | Traditional card header |
| Shield | `import { Shield } from 'lucide-react'` | Veil card header |
| X | `import { X } from 'lucide-react'` | Negative bullet points |
| Check | `import { Check } from 'lucide-react'` | Positive bullet points |

## Data Structure (Optional)
If you prefer data-driven rendering:

```js
const comparisonData = {
  traditional: {
    icon: Users,
    title: 'Traditional OTC',
    variant: 'muted',
    items: [
      { icon: X, text: 'Third party holds your funds' },
      { icon: X, text: 'Requires trust in intermediaries' },
      { icon: X, text: 'Risk of information disclosure' },
    ],
  },
  veil: {
    icon: Shield,
    title: 'Veil OTC',
    variant: 'highlighted',
    items: [
      { icon: Check, text: 'Full control of your funds' },
      { icon: Check, text: 'Trustless on-chain trading' },
      { icon: Check, text: 'Encrypted, private execution' },
    ],
  },
};
```

## Design Requirements

### Layout
- Two cards side by side
- Stack vertically on mobile
- Equal card widths

### Visual Hierarchy
- **Left card:** Muted background, subdued styling
- **Right card:** Highlighted with accent border/glow, stands out as preferred option

### Colors
- Negative icons (X): Red tone
- Positive icons (Check): Green tone
- Right card accent: Green/teal gradient or border

### Spacing
- Comfortable padding inside cards
- Gap between cards
- Vertical spacing between list items

## Notes for Claude Code
- Use your own styling system (CSS modules, Tailwind, styled-components, etc.)
- Make component accessible (semantic HTML, proper heading levels)
- Keep it responsive
- Icons should be sized appropriately (~16-20px)
