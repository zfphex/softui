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

    fn children(&mut self) -> &mut [Box<dyn Widget<'a> + 'a>] {
        self.children.as_mut_slice()
    }

    fn position_new(&mut self, parent: Rect) {
        self.size = parent.into();

        let mut x = parent.x + self.padding;
        let mut y = parent.y + self.padding;
        let mut width = 0;
        let mut height = 0;

        let widgets_remaining = self.size.remaining_widgets.unwrap_or(1);
        let (available_width, available_height) = match self.direction {
            LeftRight => {
                let width = match self.size.width {
                    Unit::Auto(_) => parent.width - 0,
                    Unit::Pixel(width) => width,
                    Unit::Percentage(_) => todo!(),
                    Unit::Em(_) => todo!(),
                };

                let height = match self.size.height {
                    Unit::Auto(_) => parent.height - 0,
                    Unit::Pixel(height) => parent.height.max(height),
                    Unit::Percentage(_) => todo!(),
                    Unit::Em(_) => todo!(),
                };
                dbg!(width, height);

                (
                    width.saturating_sub(self.padding * 2) / widgets_remaining,
                    height.saturating_sub(self.padding * 2),
                )
            }
            TopBottom => todo!(),
            // (widgets_remaining, available_height / widgets_remaining),
        };

        dbg!(&available_width, &available_height, widgets_remaining);

        let last_index = self.children.len().saturating_sub(1);
        for (i, child) in self.children.iter_mut().enumerate() {
            // child.position_new(parent);
            let name = child.name().to_string();
            let size = child.size_mut();

            // dbg!(&size);
            let width = match size.width {
                Unit::Pixel(px) => px,
                Unit::Percentage(p) => (p as f32 / 100.0 * available_width as f32).round() as usize,
                Unit::Em(_) => todo!(),
                Unit::Auto(_) => available_width,
            };

            let height = match size.height {
                Unit::Pixel(px) => px,
                Unit::Percentage(_) => todo!(),
                Unit::Em(_) => todo!(),
                Unit::Auto(_) => available_height,
            };

            dbg!(name, &size, x, y, width, height);

            child.position_new(Rect::new(x, y, width, height));

            match self.direction {
                FlexDirection::LeftRight => x += width + if i != last_index { self.gap } else { 0 },
                FlexDirection::TopBottom => y += height + if i != last_index { self.gap } else { 0 },
            }
        }
    }

    fn calculate_size(&mut self, parent: Rect) -> Size {
        let mut total_width = 0;
        let mut total_height = 0;
        let mut remaining_widgets = 0;

        if !self.children.is_empty() {
            let total_gap = self.gap * (self.children.len() - 1);
            let content_w = parent.width.saturating_sub(self.padding * 2);
            let content_h = parent.height.saturating_sub(self.padding * 2);

            match self.direction {
                FlexDirection::LeftRight => {
                    total_width += total_gap;
                    for child in &mut self.children {
                        let size = child.calculate_size(parent);
                        let (wu, hu) = (size.width, size.height);

                        //A child has something that needs a second pass.
                        if size.remaining_widgets.is_some() {
                            remaining_widgets += 1;
                        }

                        // if matches!(wu, Unit::Auto | Unit::Percentage(_))
                        //     || matches!(hu, Unit::Auto | Unit::Percentage(_))
                        // {
                        //     remaining_widgets += 1;
                        // }

                        if matches!(wu, Unit::Auto(_) | Unit::Percentage(_)) {
                            remaining_widgets += 1;
                        }

                        let w = match wu {
                            Unit::Auto(_) | Unit::Percentage(_) => 0,
                            _ => wu.to_pixels(content_w),
                        };

                        let h = match hu {
                            Unit::Auto(_) | Unit::Percentage(_) => 0,
                            _ => hu.to_pixels(content_h),
                        };

                        total_height = total_height.max(h);
                        total_width += w;
                    }
                }
                FlexDirection::TopBottom => {
                    total_height += total_gap;
                    for child in &mut self.children {
                        let size = child.calculate_size(parent);
                        let (wu, hu) = (size.width, size.height);

                        //A child has something that needs a second pass.
                        if size.remaining_widgets.is_some() {
                            remaining_widgets += 1;
                        }

                        // if matches!(wu, Unit::Auto | Unit::Percentage(_))
                        //     || matches!(hu, Unit::Auto | Unit::Percentage(_))
                        // {
                        //     remaining_widgets += 1;
                        // }

                        if matches!(hu, Unit::Auto(_) | Unit::Percentage(_)) {
                            remaining_widgets += 1;
                        }

                        let w = match wu {
                            Unit::Auto(_) | Unit::Percentage(_) => 0,
                            _ => wu.to_pixels(content_w),
                        };

                        let h = match hu {
                            Unit::Auto(_) | Unit::Percentage(_) => 0,
                            _ => hu.to_pixels(content_h),
                        };

                        total_width = total_width.max(w);
                        total_height += h;
                    }
                }
            }
        }

        // dbg!(self.name());
        // dbg!(remaining_widgets);
        let width = if remaining_widgets > 0 && self.direction == LeftRight {
            Unit::Auto(total_width + self.padding * 2)
        } else {
            Unit::Pixel(total_width + self.padding * 2)
        };

        let height = if remaining_widgets > 0 && self.direction == TopBottom {
            Unit::Auto(total_height + self.padding * 2)
        } else {
            Unit::Pixel(total_height + self.padding * 2)
        };

        self.size = Size {
            x: Unit::Pixel(0),
            y: Unit::Pixel(0),
            // width: Unit::Pixel(total_width + self.padding * 2),
            // height: Unit::Pixel(total_height + self.padding * 2),
            width,
            height,
            remaining_widgets: if remaining_widgets == 0 {
                None
            } else {
                Some(remaining_widgets)
            },
        };

        return self.size.clone();
    }

    fn position(&mut self, size: Size, parent: Rect) {
        // mini::info_raw!("{}\nsize: {:?}\n", self.name(), parent);
        // // self.size = parent.into();
        // self.size = size.clone();

        // let content_w = parent.width.saturating_sub(self.padding * 2);
        // let content_h = parent.height.saturating_sub(self.padding * 2);

        // let mut current_x = parent.x + self.padding;
        // let mut current_y = parent.y + self.padding;

        // let width = match size.width {
        //     Unit::Pixel(px) => px,
        //     Unit::Percentage(percent) => (content_w as f32 * percent as f32 / 100.0).round() as usize,
        //     Unit::Em(em) => unimplemented!(),
        //     Unit::Auto => 0,
        // };

        // let height = match size.height {
        //     Unit::Pixel(px) => px,
        //     Unit::Percentage(percent) => (content_h as f32 * percent as f32 / 100.0).round() as usize,
        //     Unit::Auto => 0,
        //     Unit::Em(_) => unimplemented!(),
        // };

        // let remaining_width = content_w - width;
        // let remaining_height = content_h - height;
        // let remaining_widgets = size.remaining_widgets.unwrap_or(1);
        // debug_assert!(remaining_widgets >= 1);

        // //IDK if this is right.
        // let (usable_width, usable_height) = match self.direction {
        //     LeftRight => (remaining_width / remaining_widgets, remaining_height),
        //     TopBottom => (remaining_width, remaining_height / remaining_widgets),
        // };

        // let last_index = self.children.len().saturating_sub(1);

        // // dbg!(self.name());
        // // dbg!(content_w, width, usable_width, remaining_widgets);

        // for (i, child) in self.children.iter_mut().enumerate() {
        //     // Resolve the child's desired Unit size against the parent's content box.
        //     // let size = child.calculate_size(parent);
        //     let size = child.size_mut().clone();
        //     // dbg!(&size, child.calculate_size(size.clone().into_rect()));
        //     // dbg!(&size, child.calculate_size(parent));

        //     mini::info_raw!("\t{}\n\twidth: {}\n\theight: {}", child.name(), size.width, size.height);

        //     let child_w = match size.width {
        //         // Unit::Auto if size.remaining_widgets.is_none() => 0,
        //         Unit::Auto => usable_width,
        //         Unit::Percentage(p) => ((p as f32 / 100.0) * content_w as f32).round() as usize,
        //         Unit::Pixel(px) => px,
        //         Unit::Em(_) => todo!(),
        //     };

        //     let child_h = match size.height {
        //         // Unit::Auto if size.remaining_widgets.is_none() => 0,
        //         Unit::Auto => usable_height,
        //         Unit::Percentage(p) => ((p as f32 / 100.0) * content_h as f32).round() as usize,
        //         Unit::Pixel(px) => px,
        //         Unit::Em(_) => todo!(),
        //     };

        //     mini::info_raw!("\twidth: {} height: {}\n", child_w, child_h);
        //     child.position(size, Rect::new(current_x, current_y, child_w, child_h));

        //     match self.direction {
        //         FlexDirection::LeftRight => current_x += child_w + if i != last_index { self.gap } else { 0 },
        //         FlexDirection::TopBottom => current_y += child_h + if i != last_index { self.gap } else { 0 },
        //     }
        // }
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
