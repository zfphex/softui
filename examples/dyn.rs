#![allow(static_mut_refs)]

use std::fmt::{self, Debug};

// ==========================================================================
// Core Types & Context
// ==========================================================================

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    /// Creates a new Color from RGB components.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Returns the red component.
    pub const fn r(&self) -> u8 {
        (self.0 >> 16) as u8
    }

    /// Returns the green component.
    pub const fn g(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    /// Returns the blue component.
    pub const fn b(&self) -> u8 {
        self.0 as u8
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Color")
            // Hex value, formatted as #RRGGBB
            .field("hex", &format_args!("#{:06X}", self.0))
            .field("rgb", &(self.r(), self.g(), self.b()))
            .finish()
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
pub const fn green() -> Color {
    Color::new(0, 128, 0)
}
pub const fn dark_gray() -> Color {
    Color::new(40, 40, 40)
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

pub static mut CTX: Option<Context> = None;
pub static mut COMMAND_QUEUE: Vec<Command> = Vec::new();

#[inline]
pub fn ctx() -> &'static mut Context {
    unsafe { CTX.as_mut().unwrap() }
}
#[derive(Debug)]
pub struct Context {
    pub mouse_pos: Rect,
    pub left_mouse_clicked: bool,
}
impl Context {
    pub fn new() -> Self {
        Self {
            mouse_pos: Rect::new(221, 16, 1, 1),
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
    pub fn clicked(&self, _area: Rect, _button: MouseButton) -> bool {
        // match button {
        //     MouseButton::Left => self.left_mouse_clicked && self.mouse_pos.intersects(area),
        //     _ => false,
        // }
        return true;
    }
}
pub fn create_ctx() -> &'static mut Context {
    unsafe {
        CTX = Some(Context::new());
        CTX.as_mut().unwrap()
    }
}

// ==========================================================================
// The Evolved `Widget` Trait
// ==========================================================================

pub trait Widget<'a>: Debug {
    fn size(&self) -> (usize, usize);
    fn layout(&mut self, area: Rect);
    fn handle_event(&mut self, ctx: &Context);
    fn draw(&self, commands: &mut Vec<Command>);

    fn on_click<F>(self, button: MouseButton, handler: F) -> OnClick<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        OnClick::new(self, button, handler)
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
    fn area_mut(&mut self) -> &mut Rect;
}

pub trait Style {
    fn set_bg(self, color: Color) -> Self;
}

pub struct OnClick<'a, W> {
    widget: W,
    handlers: Vec<(MouseButton, Box<dyn FnMut(&mut W) + 'a>)>,
}

impl<'a, W> OnClick<'a, W> {
    pub fn new(widget: W, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        let mut handlers: Vec<(MouseButton, Box<dyn FnMut(&mut W) + 'a>)> = Vec::new();
        handlers.push((button, Box::new(handler)));
        OnClick { widget, handlers }
    }

    pub fn on_click(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, Box::new(handler)));
        self
    }
}

impl<'a, W> Debug for OnClick<'a, W>
where
    W: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buttons: Vec<_> = self.handlers.iter().map(|(b, _)| b).collect();
        f.debug_struct("OnClick")
            .field("widget", &self.widget)
            .field("buttons", &buttons)
            .finish_non_exhaustive()
    }
}

impl<'a, W> Widget<'a> for OnClick<'a, W>
where
    W: Widget<'a> + Debug,
{
    fn size(&self) -> (usize, usize) {
        self.widget.size()
    }
    fn layout(&mut self, area: Rect) {
        self.widget.layout(area)
    }
    fn area_mut(&mut self) -> &mut Rect {
        self.widget.area_mut()
    }
    fn draw(&self, cmds: &mut Vec<Command>) {
        self.widget.draw(cmds)
    }
    fn handle_event(&mut self, ctx: &Context) {
        // First let the wrapped widget handle the event
        self.widget.handle_event(ctx);
        // Then run all matching handlers
        for (button, h) in &mut self.handlers {
            if ctx.clicked(*self.widget.area_mut(), *button) {
                h(&mut self.widget);
            }
        }
    }
}

// ==========================================================================
// Concrete Widget and Layout
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
    fn size(&self) -> (usize, usize) {
        (self.area.width, self.area.height)
    }
    fn layout(&mut self, area: Rect) {
        self.area = area;
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
    fn handle_event(&mut self, _ctx: &Context) {}
    fn draw(&self, commands: &mut Vec<Command>) {
        commands.push(Command {
            area: self.area,
            primative: Primative::Ellipse(0, self.bg),
        });
    }
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
macro_rules! h { ($($widget:expr),* $(,)?) => { group!($($widget),*).direction(FlexDirection::LeftRight) }; }
#[macro_export]
macro_rules! v { ($($widget:expr),* $(,)?) => { group!($($widget),*).direction(FlexDirection::TopBottom) }; }

#[macro_export]
macro_rules! group {
    ($($widget:expr),* $(,)?) => {{
        let mut children: Vec<Box<dyn Widget>> = Vec::new();
        $( children.push(Box::new($widget)); )*
        Group { children, padding: 0, gap: 0, direction: FlexDirection::default(), area: Rect::default(), bg: None }
    }};
}

#[derive(Debug, Default)]
pub struct Group<'a> {
    children: Vec<Box<dyn Widget<'a> + 'a>>,
    padding: usize,
    gap: usize,
    direction: FlexDirection,
    area: Rect,
    bg: Option<Color>,
}

impl<'a> Group<'a> {
    pub fn padding(mut self, value: usize) -> Self {
        self.padding = value;
        self
    }
    pub fn gap(mut self, value: usize) -> Self {
        self.gap = value;
        self
    }
    pub fn direction(mut self, value: FlexDirection) -> Self {
        self.direction = value;
        self
    }
}

impl<'a> Style for Group<'a> {
    fn set_bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }
}

impl<'a> Widget<'a> for Group<'a> {
    fn size(&self) -> (usize, usize) {
        let mut total_width = 0;
        let mut total_height = 0;
        if !self.children.is_empty() {
            let total_gap = self.gap * (self.children.len() - 1);
            match self.direction {
                FlexDirection::LeftRight => {
                    total_width += total_gap;
                    for child in &self.children {
                        let (w, h) = child.size();
                        total_width += w;
                        total_height = total_height.max(h);
                    }
                }
                FlexDirection::TopBottom => {
                    total_height += total_gap;
                    for child in &self.children {
                        let (w, h) = child.size();
                        total_width = total_width.max(w);
                        total_height += h;
                    }
                }
            }
        }
        (total_width + self.padding * 2, total_height + self.padding * 2)
    }
    fn layout(&mut self, area: Rect) {
        self.area = area;
        let mut current_x = area.x + self.padding;
        let mut current_y = area.y + self.padding;
        let last_index = self.children.len().saturating_sub(1);
        for (i, child) in self.children.iter_mut().enumerate() {
            let (child_w, child_h) = child.size();
            let child_area = Rect::new(current_x, current_y, child_w, child_h);
            child.layout(child_area);
            match self.direction {
                FlexDirection::LeftRight => current_x += child_w + if i != last_index { self.gap } else { 0 },
                FlexDirection::TopBottom => current_y += child_h + if i != last_index { self.gap } else { 0 },
            }
        }
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
    fn handle_event(&mut self, ctx: &Context) {
        for child in &mut self.children {
            child.handle_event(ctx);
        }
    }
    fn draw(&self, commands: &mut Vec<Command>) {
        if let Some(bg_color) = self.bg {
            commands.push(Command {
                area: self.area,
                primative: Primative::Ellipse(0, bg_color),
            });
        }
        for child in &self.children {
            child.draw(commands);
        }
    }
}

#[macro_export]
macro_rules! flex {
    ($($widget:expr),* $(,)?) => {{
        let content = group!($($widget),*);
        FlexRoot { content, margin: 0 }
    }};
}

pub struct FlexRoot<'a> {
    content: Group<'a>,
    margin: usize,
}

impl<'a> FlexRoot<'a> {
    pub fn padding(mut self, value: usize) -> Self {
        let content = std::mem::take(&mut self.content);
        self.content = content.padding(value);
        self
    }
    pub fn gap(mut self, value: usize) -> Self {
        let content = std::mem::take(&mut self.content);
        self.content = content.gap(value);
        self
    }
    pub fn margin(mut self, value: usize) -> Self {
        self.margin = value;
        self
    }
    pub fn direction(mut self, value: FlexDirection) -> Self {
        let content = std::mem::take(&mut self.content);
        self.content = content.direction(value);
        self
    }
    pub fn bg(mut self, color: Color) -> Self {
        let content = std::mem::take(&mut self.content);
        self.content = content.bg(color);
        self
    }
}

impl<'a> Drop for FlexRoot<'a> {
    fn drop(&mut self) {
        let ctx = ctx();
        let (w, h) = self.content.size();
        let total_area = Rect::new(self.margin, self.margin, w, h);

        self.content.layout(total_area);
        self.content.handle_event(ctx);

        let mut commands = Vec::new();
        self.content.draw(&mut commands);
        unsafe { COMMAND_QUEUE.extend(commands) };
    }
}

// ==========================================================================
// 5. Main Application Logic
// ==========================================================================

fn main() {
    let ctx = create_ctx();
    let mut click_count = 0;

    {
        flex!(
            // This `v!` container will be transparent by default.
            v!(rect().w(150).h(30).bg(red()), rect().w(150).h(30).bg(blue())).gap(5),
            // This `h!` container will also be transparent.
            h!(
                rect().w(40).h(65).bg(white()),
                rect()
                    .w(40)
                    .h(65)
                    .bg(blue())
                    .on_click(MouseButton::Left, |_| {
                        click_count += 1;
                        println!("Inner blue rect clicked! Count: {}", click_count);
                    })
                    .on_click(MouseButton::Right, |_| println!("right")),
            )
            .gap(5)
        )
        .padding(10)
        .gap(10)
        .margin(5)
        .bg(dark_gray()); // Set a background color only on the root container.
    }

    ctx.draw_frame();
}
