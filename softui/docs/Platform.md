```rs
pub enum Command {
    /// (x, y, width, height, radius, color)
    Ellipse(usize, usize, usize, usize, usize, Color),
    /// (x, y, width, height, color)
    Rectangle(usize, usize, usize, usize, Color),
    /// (x, y, width, height, color)
    RectangleOutline(usize, usize, usize, usize, Color),
    /// (text, font_size, x, y, Color)
    /// This needs to include the desired font.
    /// Not sure how to do that yet.
    //TODO: Should font size be f32?
    //TODO: Could change text to Cow<'_, str>
    Text(String, usize, usize, usize, Color),

    //Cannot be generic.
    CustomFn(fn(&mut Context<B>) -> ()),
}

pub static mut COMMAND_QUEUE: SegQueue<Command>> = SegQueue::new();
struct Context {
    ...
}

impl Context {
    //This function requires the current framebuffer and area.
    pub fn draw_rectangle() {...}
}
```
