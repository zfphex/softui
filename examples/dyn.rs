#![allow(static_mut_refs)]
// main.rs

use std::fmt::{self, Debug};

// ==========================================================================
// 1. Core Types
// ==========================================================================

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color(pub u32);
impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self((r as u32) << 16 | (g as u32) << 8 | (b as u32))
    }
}
pub const fn white() -> Color {
    Color::new(255, 255, 255)
}
pub const fn red() -> Color {
    Color::new(255, 0, 0)
}
pub const fn blue() -> Color {
    Color::new(0, 0, 255)
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
impl Rect {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
    pub const fn intersects(&self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Primative {
    Ellipse(usize, Color),
}

#[derive(Debug)]
pub struct Command {
    pub area: Rect,
    pub primative: Primative,
}

// ==========================================================================
// 2. Global Context and Command Queue
// ==========================================================================

pub static mut CTX: Option<Context> = None;
pub static mut COMMAND_QUEUE: Vec<Command> = Vec::new();

#[inline]
pub fn ctx() -> &'static mut Context {
    unsafe { CTX.as_mut().unwrap() }
}
#[inline]
pub fn queue_command(area: Rect, primative: Primative) {
    unsafe { COMMAND_QUEUE.push(Command { area, primative }) }
}

#[derive(Debug)]
pub struct Context {
    pub mouse_pos: Rect,
    pub left_mouse_clicked: bool,
}
impl Context {
    pub fn new() -> Self {
        Self {
            mouse_pos: Rect::new(40, 100, 1, 1),
            left_mouse_clicked: true,
        }
    }
    pub fn draw_frame(&mut self) {
        println!("--- Drawing Frame ---");
        unsafe {
            for cmd in COMMAND_QUEUE.drain(..) {
                println!("  - Drawing {:?} at {:?}", cmd.primative, cmd.area);
            }
        }
    }
    pub fn clicked(&self, area: Rect, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.left_mouse_clicked && self.mouse_pos.intersects(area),
            _ => false,
        }
    }
}
pub fn create_ctx() -> &'static mut Context {
    unsafe {
        CTX = Some(Context::new());
        CTX.as_mut().unwrap()
    }
}

// ==========================================================================
// 3. The `Widget` Trait with a Lifetime `'a`
// ==========================================================================

pub trait Widget<'a>: Debug {
    fn primative(&self) -> Primative;
    fn area(&self) -> Rect;
    fn area_mut(&mut self) -> &mut Rect;
    fn handle_event(&mut self, ctx: &Context);

    fn on_click<F>(self, _button: MouseButton, handler: F) -> OnClick<'a, Self, F>
    where
        Self: Sized,
        F: 'a + FnMut(&mut Self),
    {
        OnClick {
            widget: self,
            handler,
            button: _button,
            _phantom: std::marker::PhantomData,
        }
    }

    fn w(mut self, width: usize) -> Self
    where
        Self: Sized,
    {
        self.area_mut().width = width;
        self
    }
    fn h(mut self, height: usize) -> Self
    where
        Self: Sized,
    {
        self.area_mut().height = height;
        self
    }
    fn bg(self, color: Color) -> Self
    where
        Self: Sized,
        Self: Style,
    {
        self.set_bg(color)
    }
}

pub trait Style {
    fn set_bg(self, color: Color) -> Self;
}

pub struct OnClick<'a, W, F> {
    widget: W,
    handler: F,
    button: MouseButton,
    _phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a, W: Debug, F> Debug for OnClick<'a, W, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnClick")
            .field("widget", &self.widget)
            .field("button", &self.button)
            .finish_non_exhaustive()
    }
}
impl<'a, W, F> Widget<'a> for OnClick<'a, W, F>
where
    W: Widget<'a>,
    F: 'a + FnMut(&mut W),
{
    fn primative(&self) -> Primative {
        self.widget.primative()
    }
    fn area(&self) -> Rect {
        self.widget.area()
    }
    fn area_mut(&mut self) -> &mut Rect {
        self.widget.area_mut()
    }
    fn handle_event(&mut self, ctx: &Context) {
        self.widget.handle_event(ctx);
        if ctx.clicked(self.area(), self.button) {
            println!("  - Click detected on widget with area {:?}!", self.area());
            (self.handler)(&mut self.widget);
        }
    }
}

// ==========================================================================
// 4. Concrete Widget and Layout
// ==========================================================================

#[derive(Debug)]
pub struct Rectangle {
    pub area: Rect,
    pub bg: Color,
}
pub fn rect() -> Rectangle {
    Rectangle {
        area: Rect::new(0, 0, 10, 10),
        bg: white(),
    }
}
impl<'a> Widget<'a> for Rectangle {
    fn primative(&self) -> Primative {
        Primative::Ellipse(0, self.bg)
    }
    fn area(&self) -> Rect {
        self.area
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
    fn handle_event(&mut self, _ctx: &Context) {}
}
impl Style for Rectangle {
    fn set_bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum FlexDirection {
    #[default]
    LeftRight,
    TopBottom,
}
#[macro_export]
macro_rules! flex {
    ($($widget:expr),* $(,)?) => {{
        let mut children: Vec<Box<dyn Widget>> = Vec::new();
        $(
            children.push(Box::new($widget));
        )*
        DeferFlex {
            children,
            padding: 0,
            gap: 0,
            margin: 0,
            direction: FlexDirection::default(),
        }
    }};
}

pub struct DeferFlex<'a> {
    children: Vec<Box<dyn Widget<'a> + 'a>>,
    padding: usize,
    gap: usize,
    margin: usize,
    direction: FlexDirection,
}

impl<'a> DeferFlex<'a> {
    pub fn padding(mut self, value: usize) -> Self {
        self.padding = value;
        self
    }
    pub fn gap(mut self, value: usize) -> Self {
        self.gap = value;
        self
    }
    pub fn margin(mut self, value: usize) -> Self {
        self.margin = value;
        self
    }
    pub fn direction(mut self, value: FlexDirection) -> Self {
        self.direction = value;
        self
    }
}

impl<'a> Drop for DeferFlex<'a> {
    fn drop(&mut self) {
        println!("\n--- UI FRAME PROCESSING (DeferFlex dropped) ---");
        let ctx = ctx();

        // 1. Layout Pass
        let mut current_x = self.margin + self.padding;
        let mut current_y = self.margin + self.padding;

        let last_index = self.children.len().saturating_sub(1);

        for (i, widget) in self.children.iter_mut().enumerate() {
            let area = widget.area_mut();
            area.x = current_x;
            area.y = current_y;

            // After placing the widget, update the offset for the next widget.
            match self.direction {
                FlexDirection::LeftRight => {
                    current_x += area.width;
                    if i != last_index {
                        current_x += self.gap;
                    }
                }
                FlexDirection::TopBottom => {
                    current_y += area.height;
                    if i != last_index {
                        current_y += self.gap;
                    }
                }
            }
        }

        println!(
            "1. Layout Pass complete (direction: {:?}, padding: {}, gap: {}, margin: {}).",
            self.direction, self.padding, self.gap, self.margin
        );

        // 2. Event Pass
        println!("2. Event Pass starting...");
        for widget in &mut self.children {
            widget.handle_event(ctx);
        }
        println!("   Event Pass complete.");

        // 3. Draw Queueing Pass
        for widget in &self.children {
            queue_command(widget.area(), widget.primative());
        }
        println!("3. Draw queueing complete.");
    }
}

// ==========================================================================
// 5. Main Application Logic
// ==========================================================================

fn main() {
    let ctx = create_ctx();

    let mut click_count = 0;
    let mut blue_button_color = blue();

    println!(
        "Initial state: click_count = {}, color = {:?}",
        click_count, blue_button_color
    );

    {
        flex!(
            rect().w(100).h(50).bg(red()),
            rect()
                .w(100)
                .h(50)
                .bg(blue_button_color)
                .on_click(MouseButton::Left, |r| {
                    println!("    -> Blue rectangle's click handler executed!");
                    click_count += 1;
                    blue_button_color = white();
                    r.bg = white();
                }),
            rect().w(100).h(50).bg(Color::new(0, 255, 0))
        )
        .direction(FlexDirection::TopBottom) // Demonstrate vertical layout
        .padding(32)
        .gap(10)
        .margin(5);
    }

    ctx.draw_frame();
    println!(
        "\nFinal state: click_count = {}, color = {:?}",
        click_count, blue_button_color
    );
}
