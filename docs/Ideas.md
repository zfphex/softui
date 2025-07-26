```rust
v!(rect(), text(), image()).gap(32)

let vertical = Vertical { widgets: (Rect, Text, Image), gap: 32 }

//Layout

widgets.0.area()
widgets.1.area()
widgets.2.area()
```

Can the number of tuples inside a Vertical struct be known at compile time? What about this?

```rust
let container = Vec::new();
v!(container).gap(32)
```

