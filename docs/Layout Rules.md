```rs
Root(800)
  Group()
    Rect(50%)        // 200
    Rect(50%)        // 200
    SubGroup(Fill)   // 0 leftover
```

```rs
Group(400)
  Rect(Fill) //133
  Rect(Fill) //133
  SubGroup(Fill) //133
```

- Fixed
    - A child with fixed width always gets it.
    - Deduct from parent’s available width.
- Percentage
    - Percentages are relative to the parent’s content box width.
    - Percentages are never normalized (they can oversubscribe the parent).
    - If total % > 100%, children overflow (like CSS flex-basis).
    - Percent sizing is only allowed when the parent has a definite size (cannot be fit).
- Fill/Grow
    - After fixed and % allocations, leftover width is divided equally among all fill/grow children.
    - If leftover < 0 (oversubscribe), auto children shrink to 0.
    - Auto never steals space from % or fixed.
- Nested Containers
    - They get an allocated width from their parent.
    - Then recursively apply the same rules to their children.
- Font Relative (em)
  - TODO
  -
- Fit 
  - Fit containers need children that can size themselves without referring back to the parent.
  - Percentage and Fill children are not allowed.
- Gap
  - Gap adds space between consecutive children along the primary axis.
  - Gap is applied between children, not before the first or after the last child.
  - With N children, total gap space = gap × (N - 1).
  - Gap space is deducted from available space before distributing to Fill children.
  - Each container has its own independent gap value.
- Padding
  - Padding creates an inset space on all four sides of a container.
  - Content area = container size - (2 × padding) for each axis.
  - All child sizing modes (Fixed, Percentage, Fill, Fit) work with padding.
  - Percentage children calculate their size relative to the content area, not the container size.
  - Fit containers include their padding in the calculated size: fit size = children size + gap + (2 × padding).

```rs
Container(200px, padding: 20px)
  ├─ Content area = 200 - (2 × 20) = 160px
  └─ Child(50%)  // 50% of 160 = 80px (not 50% of 200 = 100px)

Container: 200px
┌─────────────────────────────────────┐
│  Padding: 20px                      │  
│  ┌───────────────────────────────┐  │
│  │ Content Area: 160px           │  │
│  │  ┌─────────────┐              │  │
│  │  │ Child: 80px │              │  │
│  │  │   (50%)     │              │  │
│  │  └─────────────┘              │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

