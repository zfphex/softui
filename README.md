### TODO

- [ ] Allow drawing outside of the window area without crashing. Allow for clipping.
- [x] Rectangle with rounded corners
- [ ] Framerate limiter
- [ ] Remove draw on drop and require that everything be placed into a container.

### Trait Definitions

```rs
pub enum Unit {
    Px(usize),
    ///Relative to the font-size of the element
    ///https://en.wikipedia.org/wiki/Em_(typography)
    ///https://www.w3schools.com/cssref/css_units.php
    Em(usize)
    Percentage(usize)
}

pub trait Layout {
    fn centered(self) -> Self;

    /// Distance from the left side of parent area.
    fn left<U: Into<Unit>(self, length: U) -> Self;
    /// Distance from the right side of parent area to the right side of widget.
    fn right<U: Into<Unit>>(self, length: U) -> Self;
    /// Distance from the top of parent area.
    fn top<U: Into<Unit>>(self, length: U) -> Self;
    /// Distance from the bottom of parent area to the bottom of widget.
    fn bottom<U: Into<Unit>>(self, length: U) -> Self;

    fn width<U: Into<Unit>>(self, length: U) -> Self;
    fn height<U: Into<Unit>>(self, length: U) -> Self;
    fn min_width<U: Into<Unit>>(self, length: U) -> Self;
    fn min_height<U: Into<Unit>>(self, length: U) -> Self;
    fn max_width<U: Into<Unit>>(self, length: U) -> Self;
    fn max_height<U: Into<Unit>>(self, length: U) -> Self;

    fn padding<U: Into<Unit>>(self, length: U) -> Self;
    fn padding_left<U: Into<Unit>>(self, length: U) -> Self;
    fn padding_right<U: Into<Unit>>(self, length: U) -> Self;
    fn padding_top<U: Into<Unit>>(self, length: U) -> Self;
    fn padding_bottom<U: Into<Unit>>(self, length: U) -> Self;

    fn margin<U: Into<Unit>>(self, length: U) -> Self;
    fn margin_left<U: Into<Unit>>(self, length: U) -> Self;
    fn margin_right<U: Into<Unit>>(self, length: U) -> Self;
    fn margin_top<U: Into<Unit>>(self, length: U) -> Self;
    fn margin_bottom<U: Into<Unit>>(self, length: U) -> Self;

    //TODO: What happens if the user defines both?
    //I like the idea of compile time errors for styling, css could use that.
    ///Determine the width or height automatically
    fn aspect_ratio(self, ratio: f32) -> Self;
    fn rotate(self) -> Self;
    fn transform(self) -> Self;
    //TODO: z-index, what if the user wants to render something on top or bottom?
}
```

```rs
pub trait Style {
    ///Background Color
    fn color(self, color: Color) -> Self;
    fn hex(self, hex: u32) -> Self;

    //I want the user to be able to add their own color functions to every widget.
    fn red(self) -> Self;
    fn green(self) -> Self;
    fn blue(self) -> Self;

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

```rs
pub enum Button {
    Left,
    Right,
    Middle,
    Forward,
    Back,
}

//TODO: This shouldn't be a trait. It should be a context function that is called with the desired area.

pub trait Input {
    ///Clicked and released on the UI element.
    fn clicked(&self, button: Button) -> bool;
    ///Double clicked and released on the UI element.
    fn double_click(&self, button: Button) -> bool;

    fn up(&self, button: Button) -> bool;
    fn down(&self, button: Button) -> bool;

    fn scroll_up(&self) -> bool;
    fn scroll_down(&self) -> bool;

    fn hovered(&self) -> bool;
    fn lost_focus(&self) -> bool;
    fn gained_focus(&self) -> bool;
    fn has_focus(&self) -> bool;
}
```


### Layout System

```rs
//Area will use the largest widget size unless wrapped. 
//So here width: 50, height: 100
v((button("width, height", 50, 100), button("button2", 10, 10)));

//Max width of 20
h((button("width, height", 50, 100), button("button2", 10, 10))).wrap(20, 20);

h((button(), button())).padding(10).margin(2)

struct Vertical<W> {
    widgets: W
    area: Rect,
    wrap: bool,
    //xyzw
    //top, bottom, left, right
    padding: Vec4
    margin: Vec4
}

struct Horizontal<W> {
    //... same as above
}

struct Flex {}

flex((button(), button())).direction(Direction::Vertical)
```

Immediate mode style

```rs
ui.vertical(|ui| {
    if button(ui).clicked() {
        println!("Clicked button!");
    }
});
```

Xilem style

```rs
vertical(
    button().clicked(|| println!("Clicked button!"))
)
```

### Widgets

- Container
- Text
- Button
- Slider/Progress Bar
- Radio Menu
- Check Box
- Modal Menu
- Input
- Image
- Svg
- Flexbox
- Color Picker