use crate::*;

use std::fmt::{self, Debug};

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
                width: Unit::Auto,
                height: Unit::Auto,
                remaining_widgets: None,
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

    fn position(&mut self, size: Size, prev_area: Rect) {
        self.size = prev_area.into();

        let content_w = prev_area.width.saturating_sub(self.padding * 2);
        let content_h = prev_area.height.saturating_sub(self.padding * 2);

        let mut current_x = prev_area.x + self.padding;
        let mut current_y = prev_area.y + self.padding;

        // dbg!(prev_area.width, size.width.to_pixels(content_w));
        let remaining_width = prev_area.width - size.width.to_pixels(content_w);
        let remaining_height = prev_area.height - size.height.to_pixels(content_h);
        let remaining_widgets = size.remaining_widgets.unwrap_or(1);
        let usable_width = remaining_width / remaining_widgets;
        let usable_height = remaining_height / remaining_widgets;
        let last_index = self.children.len().saturating_sub(1);

        dbg!(usable_width);

        for (i, child) in self.children.iter_mut().enumerate() {
            // Resolve the child's desired Unit size against the parent's content box.
            // let size = child.size(prev_area);
            let size = child.size_mut().clone();

            let child_w = match size.width {
                Unit::Auto => usable_width,
                Unit::Percentage(p) => ((p as f32 / 100.0) * usable_width as f32).round() as usize,
                Unit::Pixel(px) => px,
                _ => unreachable!(),
            };

            let child_h = match size.height {
                Unit::Auto => usable_height,
                Unit::Percentage(p) => ((p as f32 / 100.0) * usable_height as f32).round() as usize,
                Unit::Pixel(px) => px,
                _ => unreachable!(),
            };

            child.position(size, Rect::new(current_x, current_y, child_w, child_h));

            match self.direction {
                FlexDirection::LeftRight => current_x += child_w + if i != last_index { self.gap } else { 0 },
                FlexDirection::TopBottom => current_y += child_h + if i != last_index { self.gap } else { 0 },
            }
        }
    }

    fn calculate_size(&self, parent: Rect) -> Size {
        let mut total_width = 0;
        let mut total_height = 0;
        let mut remaining_widgets = 0;

        if !self.children.is_empty() {
            let total_gap = self.gap * (self.children.len() - 1);

            // let content_w = match self.area_new.width {
            //     Unit::Pixel(px) => px.saturating_sub(self.padding * 2),
            //     Unit::Percentage(percentage) => todo!("Group has size: {}", percentage),
            //     Unit::Auto => parent.width,
            //     _ => todo!("Assume fixed size for now"),
            // };
            // let content_h = match self.area_new.height {
            //     Unit::Pixel(px) => px.saturating_sub(self.padding * 2),
            //     Unit::Auto => parent.height,
            //     _ => todo!("Assume fixed size for now"),
            // };

            let content_w = parent.width.saturating_sub(self.padding * 2);
            let content_h = parent.height.saturating_sub(self.padding * 2);

            match self.direction {
                FlexDirection::LeftRight => {
                    total_width += total_gap;
                    for child in &self.children {
                        let size = child.calculate_size(parent);
                        let (wu, hu) = (size.width, size.height);

                        //A child has something that needs a second pass.
                        if size.remaining_widgets.is_some() {
                            remaining_widgets += 1;
                        }

                        if matches!(wu, Unit::Auto | Unit::Percentage(_))
                            || matches!(hu, Unit::Auto | Unit::Percentage(_))
                        {
                            remaining_widgets += 1;
                        }

                        let w = match wu {
                            Unit::Auto | Unit::Percentage(_) => 0,
                            _ => wu.to_pixels(content_w),
                        };

                        let h = match hu {
                            Unit::Auto | Unit::Percentage(_) => 0,
                            _ => hu.to_pixels(content_h),
                        };

                        total_height = total_height.max(h);
                        total_width += w;
                    }
                }
                FlexDirection::TopBottom => {
                    todo!()
                    // total_height += total_gap;
                    // for child in &self.children {
                    //     let size = child.size(parent);
                    //     let (wu, hu) = (size.width, size.height);

                    //     if wu == Unit::Auto || hu == Unit::Auto {
                    //         remaining_widgets += 1;
                    //     }

                    //     let w = match wu {
                    //         Unit::Auto => {
                    //             requires_second_pass = true;
                    //             0
                    //         }
                    //         _ => wu.to_pixels(content_w),
                    //     };

                    //     let h = match hu {
                    //         Unit::Auto => {
                    //             requires_second_pass = true;
                    //             0
                    //         }
                    //         _ => hu.to_pixels(content_h),
                    //     };

                    //     total_width = total_width.max(w);
                    //     total_height += h;
                    // }
                }
            }
        }

        Size {
            x: Unit::Pixel(0),
            y: Unit::Pixel(0),
            width: Unit::Pixel(total_width + self.padding * 2),
            height: Unit::Pixel(total_height + self.padding * 2),
            remaining_widgets: if remaining_widgets == 0 {
                None
            } else {
                Some(remaining_widgets)
            },
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

        let total_area = Rect::new(self.margin, self.margin, w, h);

        // self.group.layout(total_area);
        let current_size = self.group.calculate_size(total_area);
        self.group.position(current_size, total_area);

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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic() {
        let total_area = Rect::new(0, 0, 800, 600);
        let mut group = Group::new();
        group.direction = TopBottom;

        group.children.push(Box::new(rect().h_fill().w_fill()));
        group.children.push(Box::new(rect().wh(200)));
        group.children.push(Box::new(rect().h_fill().w_fill()));

        let size = group.calculate_size(total_area);

        assert_eq!(
            size,
            Size {
                x: Unit::Pixel(0),
                y: Unit::Pixel(0),
                width: Unit::Pixel(200),
                height: Unit::Pixel(200),
                remaining_widgets: Some(2),
            }
        );

        group.position(size, total_area);

        assert_eq!(*group.children[0].size_mut(), *rect().w(300).h(200).size_mut());
        assert_eq!(*group.children[1].size_mut(), *rect().y(200).wh(200).size_mut());
        assert_eq!(*group.children[2].size_mut(), *rect().y(400).w(300).h(200).size_mut());
    }

    #[test]
    fn basic_percentage() {
        let total_area = Rect::new(0, 0, 800, 600);
        let mut group = Group::new().direction(TopBottom);

        group.children.push(Box::new(rect().w(40.percent()).h(200)));
        group.children.push(Box::new(rect().w(20.percent()).h(200)));
        group.children.push(Box::new(rect().w(40.percent()).h(200)));

        let size = group.calculate_size(total_area);

        assert_eq!(
            size,
            Size {
                x: Unit::Pixel(0),
                y: Unit::Pixel(0),
                width: Unit::Pixel(320),
                height: Unit::Pixel(600),
                remaining_widgets: Some(0),
            }
        );

        group.position(size, total_area);

        assert_eq!(*group.children[0].size_mut(), *rect().y(0).w(320).h(200).size_mut());
        assert_eq!(*group.children[1].size_mut(), *rect().y(200).w(160).h(200).size_mut());
        assert_eq!(*group.children[2].size_mut(), *rect().y(400).w(320).h(200).size_mut());
    }

    #[test]
    fn basic_subgroup() {
        let total_area = Rect::new(0, 0, 800, 600);
        let mut group = Group::new().direction(TopBottom);
        let mut subgroup = Group::new().direction(TopBottom);

        subgroup.children.push(Box::new(rect().wh(100)));

        group.children.push(Box::new(subgroup));
        group.children.push(Box::new(rect().wh(200)));

        let size = group.calculate_size(total_area);

        assert_eq!(
            size,
            Size {
                x: Unit::Pixel(0),
                y: Unit::Pixel(0),
                width: Unit::Pixel(200),
                height: Unit::Pixel(300),
                remaining_widgets: Some(0),
            }
        );

        group.position(size, total_area);

        assert_eq!(*group.children[0].size_mut(), *rect().y(0).wh(100).size_mut());
        assert_eq!(*group.children[1].size_mut(), *rect().y(100).wh(200).size_mut());
    }

    #[test]
    fn complex_subgroup() {
        let total_area = Rect::new(0, 0, 800, 600);
        let mut group = Group::new().direction(TopBottom);

        let mut subgroup = Group::new().direction(TopBottom);

        subgroup.children.push(Box::new(rect().w(25.percent()).h(200)));
        subgroup.children.push(Box::new(rect().w(50.percent()).h(200)));
        subgroup.children.push(Box::new(rect().w(25.percent()).h(200)));

        group.children.push(Box::new(subgroup));
        // group.children.push(Box::new(rect().wh(200)));

        let size = group.calculate_size(total_area);
        dbg!(size);

        // assert_eq!(
        //     size,
        //     Size {
        //         width: Unit::Pixel(200),
        //         height: Unit::Pixel(300),
        //         remaining_widgets: Some(0),
        //     }
        // );

        // group.layout_new(size, total_area);

        // assert_eq!(*group.children[0].area_mut(), *rect().y(0).wh(100).area_mut());
        // assert_eq!(*group.children[1].area_mut(), *rect().y(100).wh(200).area_mut());
    }
}
