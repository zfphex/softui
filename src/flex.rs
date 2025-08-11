use crate::*;

use std::fmt::{self, Debug};

pub enum MouseAction {
    Pressed,
    Released,
    Clicked,
}

pub fn convert_button_to_state(ctx: &mut Context, button: MouseButton) -> MouseButtonState {
    match button {
        MouseButton::Left => ctx.window.left_mouse,
        MouseButton::Right => ctx.window.right_mouse,
        MouseButton::Middle => ctx.window.middle_mouse,
        MouseButton::Mouse4 => ctx.window.mouse_4,
        MouseButton::Mouse5 => ctx.window.mouse_5,
    }
}

pub fn clicked(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    convert_button_to_state(ctx, button).clicked(area)
}

pub fn pressed(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    convert_button_to_state(ctx, button).is_pressed()
}

pub fn released(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    convert_button_to_state(ctx, button).is_released()
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
    fn gap(mut self, gap: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.gap(gap);
        self
    }
    fn margin(mut self, margin: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.margin(margin);
        self
    }
    fn padding(mut self, padding: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.padding(padding);
        self
    }
    fn direction(mut self, direction: FlexDirection) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.direction(direction);
        self
    }
    fn size(&self) -> (usize, usize) {
        self.widget.size()
    }
    fn layout(&mut self, area: Rect) {
        self.widget.layout(area)
    }
    fn desired_size(&self) -> (Unit, Unit) {
        self.widget.desired_size()
    }
    fn area_mut_new(&mut self) -> &mut UnitRect {
        self.widget.area_mut_new()
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
        FlexRoot { group: group!($($widget),*), margin: 0 }
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
            area_new: $crate::UnitRect::default(),
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
    pub area_new: UnitRect,
    pub bg: Option<Color>,
}

impl<'a> Group<'a> {
    pub fn new() -> Self {
        Group {
            children: Vec::new(),
            padding: 0,
            gap: 0,
            direction: FlexDirection::default(),
            area: Rect::default(),
            area_new: UnitRect::default(),
            bg: None,
        }
    }
}

impl<'a> Widget<'a> for Group<'a> {
    fn gap(mut self, gap: usize) -> Self
    where
        Self: Sized,
    {
        self.gap = gap;
        self
    }
    fn padding(mut self, padding: usize) -> Self
    where
        Self: Sized,
    {
        self.padding = padding;
        self
    }
    fn direction(mut self, direction: FlexDirection) -> Self
    where
        Self: Sized,
    {
        self.direction = direction;
        self
    }

    fn area_mut_new(&mut self) -> &mut UnitRect {
        &mut self.area_new
    }

    fn desired_size(&self) -> (Unit, Unit) {
        // (self.area_new.width, self.area_new.height)
        let mut total_width = 0;
        let mut total_height = 0;

        if !self.children.is_empty() {
            let total_gap = self.gap * (self.children.len() - 1);

            let content_w = match self.area_new.width {
                Unit::Pixel(px) => px.saturating_sub(self.padding * 2),
                _ => todo!("Assume fixed size for now"),
            };
            let content_h = match self.area_new.height {
                Unit::Pixel(px) => px.saturating_sub(self.padding * 2),
                _ => todo!("Assume fixed size for now"),
            };

            match self.direction {
                FlexDirection::LeftRight => {
                    total_width += total_gap;
                    for child in &self.children {
                        // let (wu, hu) = child.desired_size();
                        // let w = wu.to_pixels(content_w);
                        // let h = hu.to_pixels(content_h);
                        // total_width += w;
                        // total_height = total_height.max(h);
                        todo!();
                    }
                }
                FlexDirection::TopBottom => {
                    total_height += total_gap;
                    for child in &self.children {
                        let (wu, hu) = child.desired_size();
                        // let w = wu.to_pixels(content_w);
                        // let h = hu.to_pixels(content_h);

                        let w = match wu {
                            Unit::Auto => 0,
                            _ => wu.to_pixels(content_w),
                        };
                        let h = match hu {
                            Unit::Auto => 0,
                            _ => hu.to_pixels(content_h),
                        };
                        total_width = total_width.max(w);
                        total_height += h;
                    }
                }
            }
        }

        // (todo!(), todo!())
        (
            Unit::Pixel(total_width + self.padding * 2),
            Unit::Pixel(total_height + self.padding * 2),
        )
    }

    fn size(&self) -> (usize, usize) {
        let mut total_width = 0;
        let mut total_height = 0;

        if !self.children.is_empty() {
            let total_gap = self.gap * (self.children.len() - 1);
            let content_w = self.area.width.saturating_sub(self.padding * 2);
            let content_h = self.area.height.saturating_sub(self.padding * 2);

            match self.direction {
                FlexDirection::LeftRight => {
                    total_width += total_gap;
                    for child in &self.children {
                        let (wu, hu) = child.desired_size();
                        let w = wu.to_pixels(content_w);
                        let h = hu.to_pixels(content_h);
                        total_width += w;
                        total_height = total_height.max(h);
                    }
                }
                FlexDirection::TopBottom => {
                    total_height += total_gap;
                    for child in &self.children {
                        let (wu, hu) = child.desired_size();
                        let w = wu.to_pixels(content_w);
                        let h = hu.to_pixels(content_h);
                        total_width = total_width.max(w);
                        total_height += h;
                    }
                }
            }
        }

        (total_width + self.padding * 2, total_height + self.padding * 2)
    }

    //TODO: I don't think this takes into account the remaing size.
    fn layout(&mut self, area: Rect) {
        self.area = area;
        self.area_new = area.into();

        let content_w = area.width.saturating_sub(self.padding * 2);
        let content_h = area.height.saturating_sub(self.padding * 2);

        let mut current_x = area.x + self.padding;
        let mut current_y = area.y + self.padding;
        let last_index = self.children.len().saturating_sub(1);

        for (i, child) in self.children.iter_mut().enumerate() {
            // Resolve the child's desired Unit size against the parent's content box.
            let (wu, hu) = child.desired_size();
            dbg!(child.name());
            dbg!(child.desired_size());

            //If a child needs to fill the remaing space, the space is not calculated yet
            let child_w = match wu {
                Unit::Auto => 0,
                _ => wu.to_pixels(content_w),
            };
            let child_h = match hu {
                Unit::Auto => 0,
                _ => hu.to_pixels(content_h),
            };

            child.layout(Rect::new(current_x, current_y, child_w, child_h));

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
    //TODO: Style is not getting through here.
    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        if let Some(bg) = self.bg {
            commands.push(Command {
                area: self.area,
                primative: Primative::Ellipse(0, bg),
            });
        }

        for child in &self.children {
            child.draw(commands, child.style());
        }
    }
}

pub struct FlexRoot<'a> {
    //TODO: When someone calls .bg() on Group the type is changed into StyledWidget.
    //Since FlexRoot is not a widget, the widget is forced back into a group
    //Flex<Group<StyledWidget<Group>>>
    //The style is lost somewhere through the propagation chain.
    //content could probably be swapped for a generic widget.
    pub group: Group<'a>,
    pub margin: usize,
}

//TODO: Re-work builder on flex root.
impl<'a> FlexRoot<'a> {
    pub fn padding(mut self, padding: usize) -> Self {
        self.group.padding = padding;
        self
    }
    pub fn gap(mut self, gap: usize) -> Self {
        self.group.gap = gap;
        self
    }
    pub fn margin(mut self, margin: usize) -> Self {
        self.margin = margin;
        self
    }
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.group.direction = direction;
        self
    }
    pub fn bg(mut self, color: Color) -> Self {
        self.group.bg = Some(color);
        self
    }
}

impl<'a> Drop for FlexRoot<'a> {
    fn drop(&mut self) {
        let ctx = ctx();
        let (w, h) = (ctx.window.width(), ctx.window.height());
        let total_area = Rect::new(self.margin, self.margin, w, h);

        self.group.layout(total_area);
        self.group.handle_event(ctx);

        let mut commands = Vec::new();

        self.group.draw(&mut commands, None);

        for command in commands {
            mini::info!("{:#?}", command);
            unsafe { COMMAND_QUEUE.push(command) };
        }
    }
}
