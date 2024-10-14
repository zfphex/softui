- [ ] Improved layout
- [ ] Better widgets, lists, button
- [ ] Styling, rounded borders, em sizing.
- [ ] Layout slices and mutable slices...
- [ ] Lines widget with line wrapping (how to handle clicking different lines???)
- [ ] Cross API event crate with easy conversion.
- [ ] Native MacOS window support.

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
