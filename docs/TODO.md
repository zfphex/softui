- [ ] Layout slices and mutable slices...
- [ ] Better widgets, lists, button
- [ ] Styling, rounded borders, em sizing.
- [ ] Lines widget with line wrapping (how to handle clicking different lines???)
- [ ] Frame-rate limiter
- [ ] The custom primative functions do not allow for any arguments such as radius on area, so it's not very useful.

I like the idea of a trait that you pull in that has preset margin and padding values.
I remember someone had a css framework that had mathematically calculated margins and it looked very nice.
I literally cannot find it though.

```rs
use softui::Spacing;

rect().margin_s()
rect().margin_m()
rect().margin_l()
rect().margin_xl()
```
