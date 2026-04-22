One of the problems with immediate mode ui is that in order to hit test complex layouts, you need to cache the previous fame. This raises a larger issue of matching the previous frames elements to the current. How can I know which element is which if the amount of elements has changed?

You can use location, i.e. file and line number, but then what happens in this case:

```rust
for _ in 0..5 {
    //Multiple elements in the same location.
    if text(format!("Button {i}")).clicked() {
    }
}
```

You can hash the text or label of each ui element but what about for clashes or what if the label changes, doesn't really make any sense.