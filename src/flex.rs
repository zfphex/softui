use crate::*;

use std::fmt::{self, Debug};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    #[default]
    LeftRight,
    RightLeft,
    TopBottom,
    BottomTop,
}

#[macro_export]
macro_rules! flex {
    ($($widget:expr),* $(,)?) => {{
        $crate::FlexRoot { group: $crate::group!($($widget),*), margin: 0 }
    }};
}

#[macro_export]
macro_rules! h { ($($widget:expr),* $(,)?) => { $crate::group!($($widget),*).direction($crate::FlexDirection::LeftRight) }; }

#[macro_export]
macro_rules! v { ($($widget:expr),* $(,)?) => { $crate::group!($($widget),*).direction($crate::FlexDirection::TopBottom) }; }

#[macro_export]
macro_rules! group {
    ($($widget:expr),* $(,)?) => {{
        let mut group = $crate::Group::new();

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
            direction: FlexDirection::LeftRight,
            size: Size {
                x: Unit::Pixel(0),
                y: Unit::Pixel(0),
                width: Unit::Fill { used: 0 },
                height: Unit::Fill { used: 0 },
                widgets_left: None,
            },
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

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }

    fn children(&mut self) -> &mut [Box<dyn Widget<'a> + 'a>] {
        self.children.as_mut_slice()
    }

    fn is_container(&self) -> bool {
        true
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

    fn position(&mut self, area: Rect) {
        todo!()
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

        //First pass caculates the space with a fixed size.
        self.group.size(window);

        //Second pass calculates the size of the auto/relative percentage widgets.
        self.group.size(window);

        //Set the (x, y) position of each widget.
        self.group.position(window);
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
