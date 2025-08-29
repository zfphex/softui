```js
Root (800)
  Group
    Rect(50%)        // 200
    Rect(50%)        // 200
    SubGroup(auto)   // 0 leftover
```

```js
Group(400)
  Rect(auto) //133
  Rect(auto) //133
  SubGroup(auto) //133
```

- Fixed
    - A child with fixed width always gets it.
    - Deduct from parent’s available width.
- Percentage
    - Percentages are relative to the parent’s content box width.
    - Percentages are never normalized (they can oversubscribe the parent).
    - If total % > 100%, children overflow (like CSS flex-basis).
- Auto
    - After fixed and % allocations, leftover width is divided equally among all auto children.
    - If leftover < 0 (oversubscribe), auto children shrink to 0.
    - Auto never steals space from % or fixed.
- Nested Containers
    - They get an allocated width from their parent.
    - Then recursively apply the same rules to their children.



