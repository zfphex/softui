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

```rs
// width, height = 800, 600
//A rectangle with a width of 10% of 800 (80px), height of 20px
flex!(rect().w(10.percent()).h(20))
```