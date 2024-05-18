Trait Definitions

```rs
pub enum Unit {
    Px(usize),
    ///Relative to the font-size of the element
    ///https://en.wikipedia.org/wiki/Em_(typography)
    /// https://www.w3schools.com/cssref/css_units.php
    Em(usize)
    Percentage(usize)
    Float(f32),
}

pub trait Layout {
    fn centered(self) -> Self;
    fn left(self, length: Unit) -> Self;
    fn right(self, length: Unit) -> Self;
    fn top(self, length: Unit) -> Self;
    fn bottom(self, length: Unit) -> Self;
    fn width(self, length: Unit) -> Self;
    fn height(self, length: Unit) -> Self;
    //TODO: What happens if the user defines both?
    //I like the idea of compile time errors for styling, css could use that.
    ///Determine the width or height automatically
    fn aspect_ratio(self, ratio: f32) -> Self;
    fn max_width(self, length: Unit) -> Self;
    fn max_height(self, length: Unit) -> Self;
    fn min_width(self, length: Unit) -> Self;
    fn min_height(self, length: Unit) -> Self;
    fn padding(self, length: Unit) -> Self;
    fn padding_left(self, length: Unit) -> Self;
    fn padding_right(self, length: Unit) -> Self;
    fn padding_top(self), length: Unit -> Self;
    fn padding_bottom(self, length: Unit) -> Self;
    fn margin_left(self, length: Unit) -> Self;
    fn margin_right(self, length: Unit) -> Self;
    fn margin_top(self, length: Unit) -> Self;
    fn margin_bottom(self, length: Unit) -> Self;
    //TODO: z-index, what if the user wants to render something on top or bottom?

    fn rotate(self) -> Self;
    fn transform(self) -> Self;
}
```

```rs
pub trait Style {
    ///Background Color
    fn color(self, color: Color) -> Self;
    //TODO: I'm not really sure what I want here. Maybe gradients?
}
```

```rs
pub trait Text, { 
    //There are lots of ways the user might want text to wrap.
    fn text_wrap(self) -> Self;
    fn color(self, color: Color) -> Self;
    fn outline_color(self, color: Color) -> Self;
    fn size(self, size: usize) -> Self;
    fn bold(self) -> Self;
    fn italic(self) -> Self;
    fn underlined(self) -> Self;
}
```
