use crate::*;

use std::fmt::{self, Debug};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
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
        let mut group = Group::new();

        $(
            group.children.push(Box::new($widget));
        )*

        group
    }};
}

#[derive(Debug)]
pub struct Group<'a> {
    //TODO: Does this need to be Boxed?
    pub children: Vec<Box<dyn Widget<'a> + 'a>>,
    pub size: Size,
    //Private for better LSP support.
    bg: Option<Color>,
    gap: usize,
    direction: FlexDirection,
    padding: usize,
}

impl<'a> Group<'a> {
    pub fn new() -> Self {
        Group {
            children: Vec::new(),
            padding: 0,
            gap: 0,
            direction: FlexDirection::default(),
            size: Size {
                x: Unit::Pixel(0),
                y: Unit::Pixel(0),
                width: Unit::Auto(0),
                height: Unit::Auto(0),
                widgets_left: None,
            },
            bg: None,
        }
    }
}

pub static mut RECURSE: usize = 0;

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

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }

    fn children(&mut self) -> &mut [Box<dyn Widget<'a> + 'a>] {
        self.children.as_mut_slice()
    }

    fn is_container(&self) -> bool {
        true
    }

    fn size(&mut self, parent: Rect) {
        let mut total_width = 0;
        let mut total_height = 0;
        let mut widgets_left = self.size.widgets_left.unwrap_or(0);

        let parent_width = parent.width - (self.padding * 2);
        let parent_height = parent.height - (self.padding * 2);

        if !self.children.is_empty() {
            total_width += self.gap * (self.children.len() - 1);
        }

        let is_second_pass = self.size.widgets_left.is_some();

        let container_width = match self.size.width {
            Unit::Pixel(px) => px,
            Unit::Auto(used) => parent_width - used,
            _ if is_second_pass => unreachable!(),
            _ => 0,
        };

        let container_height = match self.size.height {
            Unit::Pixel(px) => px,
            _ if is_second_pass => unreachable!(),
            _ => 0,
        };

        // dbg!(container_width, container_height, &self.size.width);

        //TODO: This should probably be part of the function since it's important for debugging.
        unsafe { RECURSE += 1 };

        print!("{}", "\t".repeat(unsafe { RECURSE - 1 }));
        println!("Parent: {}", self.name());

        print!("{}", "\t".repeat(unsafe { RECURSE - 1 }));
        println!("{:?}", self.size);

        unsafe { RECURSE += 1 };

        for child in &mut self.children {
            let is_container = child.is_container();
            print!("{}", "\t".repeat(unsafe { RECURSE - 1 }));
            println!("Child: {}", child.name());
            print!("{}", "\t".repeat(unsafe { RECURSE - 1 }));
            println!("{:?}", child.size_mut());
            print!("{}", "\t".repeat(unsafe { RECURSE - 1 }));
            println!("{} {}", parent_width, parent_height);

            if is_container {
                child.size(Rect::new(0, 0, container_width, container_height));
            } else {
                //Doesn't matter what we pass in at all.
                child.size(Rect::new(0, 0, 0, 0));
            }

            let size = child.size_mut();

            if !is_second_pass {
                match size.width {
                    Unit::Pixel(px) => total_width += px,
                    Unit::Em(_) => todo!(),
                    _ => widgets_left += 1,
                }

                match size.height {
                    Unit::Pixel(px) => total_height = total_height.max(px),
                    Unit::Em(_) => todo!(),
                    _ => widgets_left += 1,
                }
            } else {
                let width_free = if is_container {
                    container_width / widgets_left
                } else {
                    container_width
                };
                // let height_free = if is_container {
                //     container_height / widgets_left
                // } else {
                //     container_height
                // };
                let height_free = container_height;
                let width = match size.width {
                    Unit::Pixel(px) => px,
                    Unit::Percentage(p) => (width_free as f32 * p as f32 / 100.0).round() as usize,
                    // Unit::Auto(_) if is_container => width_free / widgets_left,
                    Unit::Auto(_) => width_free,
                    Unit::Em(_) => todo!(),
                };

                //TODO: IDK what height is supposed to be for LeftRight layouts.
                let height = match size.height {
                    Unit::Pixel(px) => px,
                    Unit::Percentage(p) => (height_free as f32 * p as f32 / 100.0).round() as usize,
                    // Unit::Auto(_) if is_container => height_free / widgets_left,
                    Unit::Auto(_) => height_free,
                    Unit::Em(_) => todo!(),
                };

                // dbg!(size.width, width_free, widgets_left);

                total_width += width;
                total_height = total_height.max(height);

                size.width = width.pixel();
                size.height = height.pixel();
            }
        }

        let width = if is_second_pass {
            Unit::Pixel(total_width)
        } else {
            Unit::Auto(total_width)
        };

        self.size = size(0, 0, width, total_height);
        if widgets_left >= 1 {
            self.size.widgets_left = Some(widgets_left);
        }
    }

    fn position(&mut self, size: Size, parent: Rect) {
        // mini::info_raw!("{}\nsize: {:?}\n", self.name(), parent);
        // // self.size = parent.into();
        // self.size = size.clone();

        let content_w = parent.width.saturating_sub(self.padding * 2);
        let content_h = parent.height.saturating_sub(self.padding * 2);

        let mut current_x = parent.x + self.padding;
        let mut current_y = parent.y + self.padding;

        let last_index = self.children.len().saturating_sub(1);

        for (i, child) in self.children.iter_mut().enumerate() {
            let size = child.size_mut().clone();

            let child_w = match size.width {
                Unit::Pixel(px) => px,
                _ => unreachable!(),
            };

            let child_h = match size.height {
                Unit::Pixel(px) => px,
                _ => unreachable!(),
            };

            child.position(
                unit::size(0, 0, 0, 0),
                Rect::new(current_x, current_y, child_w, child_h),
            );

            match self.direction {
                FlexDirection::LeftRight => current_x += child_w + if i != last_index { self.gap } else { 0 },
                FlexDirection::TopBottom => current_y += child_h + if i != last_index { self.gap } else { 0 },
            }
        }
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
                area: todo!(),
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
        let (w, h) = (ctx_width(), ctx_height());

        let window = Rect::new(self.margin, self.margin, w, h);

        let _ = self.group.size(window);
        self.group.position(size(0, 0, 0, 0), window);
        // let current_size = self.group.calculate_size(total_area);
        // self.group.position(current_size, total_area);

        //TODO: Remove context from handle_event and use atomic mouse state :)
        let ctx = unsafe { ctx() };
        self.group.handle_event(ctx);

        let mut commands = Vec::new();

        self.group.draw(&mut commands, None);

        for command in commands {
            mini::info!("{:#?}", command);
            unsafe { COMMAND_QUEUE.push(command) };
        }
    }
}
