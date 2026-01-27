# OTC Comparison Section - Background Decoration

This file contains the removed background SVG decoration for the OTC comparison section.

## How to restore

1. Open `frontend/app/page.tsx`
2. Find the OTC Comparison Section (search for `{/* OTC Comparison Section */}`)
3. Inside the `<section>` tag, add the SVG below as the first child (before the `<div className="max-w-6xl...">`)

## Code

```tsx
{/* Background Stacks Decoration */}
<svg
  className="absolute inset-0 w-full h-full pointer-events-none"
  viewBox="0 0 1200 600"
  preserveAspectRatio="xMidYMid slice"
>
  {/* Centered stacks - continuous bar chart around center (600) */}
  {/* Traditional OTC side - 3 stacks with 4 lines each (white/muted) */}
  {/* Column 1 - leftmost */}
  <g stroke="rgba(255,255,255,0.15)" strokeWidth="2" strokeLinecap="round">
    <line x1="280" y1="535" x2="340" y2="535" />
    <line x1="280" y1="550" x2="340" y2="550" />
    <line x1="280" y1="565" x2="340" y2="565" />
    <line x1="280" y1="580" x2="340" y2="580" />
  </g>

  {/* Column 2 */}
  <g stroke="rgba(255,255,255,0.15)" strokeWidth="2" strokeLinecap="round">
    <line x1="390" y1="535" x2="450" y2="535" />
    <line x1="390" y1="550" x2="450" y2="550" />
    <line x1="390" y1="565" x2="450" y2="565" />
    <line x1="390" y1="580" x2="450" y2="580" />
  </g>

  {/* Column 3 - near center */}
  <g stroke="rgba(255,255,255,0.15)" strokeWidth="2" strokeLinecap="round">
    <line x1="500" y1="535" x2="560" y2="535" />
    <line x1="500" y1="550" x2="560" y2="550" />
    <line x1="500" y1="565" x2="560" y2="565" />
    <line x1="500" y1="580" x2="560" y2="580" />
  </g>

  {/* Veil OTC side - 3 growing stacks (orange) */}
  {/* Column 4 - near center, 6 lines */}
  <g stroke="rgba(249,115,22,0.4)" strokeWidth="2" strokeLinecap="round">
    <line x1="640" y1="505" x2="700" y2="505" />
    <line x1="640" y1="520" x2="700" y2="520" />
    <line x1="640" y1="535" x2="700" y2="535" />
    <line x1="640" y1="550" x2="700" y2="550" />
    <line x1="640" y1="565" x2="700" y2="565" />
    <line x1="640" y1="580" x2="700" y2="580" />
  </g>

  {/* Column 5 - 8 lines */}
  <g stroke="rgba(249,115,22,0.4)" strokeWidth="2" strokeLinecap="round">
    <line x1="750" y1="475" x2="810" y2="475" />
    <line x1="750" y1="490" x2="810" y2="490" />
    <line x1="750" y1="505" x2="810" y2="505" />
    <line x1="750" y1="520" x2="810" y2="520" />
    <line x1="750" y1="535" x2="810" y2="535" />
    <line x1="750" y1="550" x2="810" y2="550" />
    <line x1="750" y1="565" x2="810" y2="565" />
    <line x1="750" y1="580" x2="810" y2="580" />
  </g>

  {/* Column 6 - rightmost, 12 lines */}
  <g stroke="rgba(249,115,22,0.4)" strokeWidth="2" strokeLinecap="round">
    <line x1="860" y1="415" x2="920" y2="415" />
    <line x1="860" y1="430" x2="920" y2="430" />
    <line x1="860" y1="445" x2="920" y2="445" />
    <line x1="860" y1="460" x2="920" y2="460" />
    <line x1="860" y1="475" x2="920" y2="475" />
    <line x1="860" y1="490" x2="920" y2="490" />
    <line x1="860" y1="505" x2="920" y2="505" />
    <line x1="860" y1="520" x2="920" y2="520" />
    <line x1="860" y1="535" x2="920" y2="535" />
    <line x1="860" y1="550" x2="920" y2="550" />
    <line x1="860" y1="565" x2="920" y2="565" />
    <line x1="860" y1="580" x2="920" y2="580" />
  </g>
</svg>
```

## Design notes

- White stacked bars on the left (Traditional OTC) - flat, same height
- Orange stacked bars on the right (Veil OTC) - growing progressively taller
- Represents growth/improvement on the Veil side vs stagnation on Traditional
