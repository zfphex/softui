use crate::*;

use std::fmt::{self, Debug};

pub enum MouseAction {
    Pressed,
    Released,
    Clicked,
}

pub fn clicked(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    match button {
        MouseButton::Left => ctx.window.left_mouse.clicked(area),
        MouseButton::Right => ctx.window.right_mouse.clicked(area),
        MouseButton::Middle => ctx.window.middle_mouse.clicked(area),
        MouseButton::Mouse4 => ctx.window.mouse_4.clicked(area),
        MouseButton::Mouse5 => ctx.window.mouse_5.clicked(area),
    }
}

pub fn pressed(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    match button {
        MouseButton::Left => ctx.window.left_mouse.is_pressed(),
        MouseButton::Right => ctx.window.right_mouse.is_pressed(),
        MouseButton::Middle => ctx.window.middle_mouse.is_pressed(),
        MouseButton::Mouse4 => ctx.window.mouse_4.is_pressed(),
        MouseButton::Mouse5 => ctx.window.mouse_5.is_pressed(),
    }
}

pub fn released(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    match button {
        MouseButton::Left => ctx.window.left_mouse.is_released(),
        MouseButton::Right => ctx.window.right_mouse.is_released(),
        MouseButton::Middle => ctx.window.middle_mouse.is_released(),
        MouseButton::Mouse4 => ctx.window.mouse_4.is_released(),
        MouseButton::Mouse5 => ctx.window.mouse_5.is_released(),
    }
}

pub struct Click<'a, W> {
    widget: W,
    handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
    style: Option<Style>,
}

#[rustfmt::skip] 
impl<'a, W> Click<'a, W> {
    pub fn new(style: Option<Style>, widget: W, button: MouseButton, action: MouseAction, handler: impl FnMut(&mut W) + 'a) -> Self {
        let mut handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)> = Vec::new();
        handlers.push((button, action, Box::new(handler)));
        Click { widget, handlers, style}
    }

    pub fn on_click(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Clicked, Box::new(handler)));
        self
    }

    pub fn on_press(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Pressed, Box::new(handler)));
        self
    }

    pub fn on_release(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a,) -> Self {
        self.handlers.push((button, MouseAction::Released, Box::new(handler)));
        self
    }
}

impl<'a, W> Debug for Click<'a, W>
where
    W: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buttons: Vec<_> = self.handlers.iter().map(|(b, _, _)| b).collect();
        f.debug_struct("OnClick")
            .field("widget", &self.widget)
            .field("buttons", &buttons)
            .finish_non_exhaustive()
    }
}

impl<'a, W> Widget<'a> for Click<'a, W>
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
    fn draw(&self, cmds: &mut Vec<Command>, style: Option<Style>) {
        self.widget.draw(cmds, self.style)
    }
    fn style(&self) -> Option<Style> {
        self.widget.style()
    }
    fn handle_event(&mut self, ctx: &mut Context) {
        // First let the wrapped widget handle the event
        self.widget.handle_event(ctx);

        // Then run all matching handlers
        for (button, action, h) in &mut self.handlers {
            if match action {
                MouseAction::Pressed => pressed(ctx, *self.widget.area_mut(), *button),
                MouseAction::Released => released(ctx, *self.widget.area_mut(), *button),
                MouseAction::Clicked => clicked(ctx, *self.widget.area_mut(), *button),
            } {
                h(&mut self.widget);
            }
        }
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
        FlexRoot { content: group!($($widget),*), margin: 0 }
    }};
}

#[macro_export]
macro_rules! h { ($($widget:expr),* $(,)?) => { $crate::group!($($widget),*).direction($crate::FlexDirection::LeftRight) }; }

#[macro_export]
macro_rules! v { ($($widget:expr),* $(,)?) => { $crate::group!($($widget),*).direction($crate::FlexDirection::TopBottom) }; }

#[macro_export]
macro_rules! group {
    ($($widget:expr),* $(,)?) => {{
        let mut group = $crate::Group {
            children: Vec::new(),
            padding: 0,
            gap: 0,
            direction: $crate::FlexDirection::default(),
            area: $crate::Rect::default(),
            bg: None,
        };

        $(
            group.children.push(Box::new($widget));
        )*

        group
    }};
}

#[derive(Debug, Default)]
pub struct Group<'a> {
    //TODO: Does this need to be Boxed?
    pub children: Vec<Box<dyn Widget<'a> + 'a>>,
    pub padding: usize,
    pub gap: usize,
    pub direction: FlexDirection,
    pub area: Rect,
    pub bg: Option<Color>,
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
    fn handle_event(&mut self, ctx: &mut Context) {
        for child in &mut self.children {
            child.handle_event(ctx);
        }
    }
    fn draw(&self, commands: &mut Vec<Command>, _: Option<Style>) {
        if let Some(bg_color) = self.bg {
            commands.push(Command {
                area: self.area,
                primative: Primative::Ellipse(0, bg_color),
            });
        }

        for child in &self.children {
            child.draw(commands, child.style());
        }
    }
}

pub struct FlexRoot<'a> {
    pub content: Group<'a>,
    pub margin: usize,
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
        self.content.bg = Some(color);
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
        //TODO: FlexRoot has no styling options...
        self.content.draw(&mut commands, None);

        for command in commands {
            unsafe { COMMAND_QUEUE.push(command) };
        }
    }
}
