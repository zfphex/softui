//! New layout system starting with something akin to flexbox.
//!

//There should be two types of flex, one for vertical and one for horizontal

use crate::{Command, Direction, Primative, Rect, Widget};

#[derive(Default, Debug)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Default, Debug)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Default, Debug)]
pub struct Flex {
    pub direction: Direction,
    pub halign: HorizontalAlignment,
    pub valign: VerticalAlignment,
    pub gap: usize,
    pub fill: bool,
}

fn v() -> Flex {
    Flex {
        direction: Direction::Vertical,
        halign: HorizontalAlignment::Left,
        valign: VerticalAlignment::Top,
        gap: 0,
        fill: false,
    }
}

fn h() -> Flex {
    Flex {
        direction: Direction::Horizontal,
        ..Default::default()
    }
}

// pub fn flex<T: Widget>(widget: *mut T, state: &mut FlexState) {
//     let widget = unsafe { widget.as_mut().unwrap() };
// }

// pub struct FlexState {}

#[derive(Debug)]
pub struct Segment {
    pub direction: Direction,
    ///Either the total height or width.
    ///Depends on the direction.
    pub size: usize,
    ///Max width or max height
    pub max: usize,
    pub widgets: Vec<(Rect, Primative)>,
}

pub fn flex(widgets: &[(Rect, Primative)]) {
    for widget in widgets {}
}

/// Give a warning to the user if they pass something in that does not implement `Widget`.
/// This function basically strips the types out so they're easier to work with.
pub fn widget<T: Widget>(mut widget: T) -> (Rect, Primative) {
    let widgets = widget.as_uniform_layout_type();

    if widgets.len() == 1 {
        let area = widget.area();
        let primative = widget.primative();
        (area, primative)
    } else {
        todo!("Not sure how to do this...probably just need to use a vector :(")
    }
}

//There is no easy way to concatinate slices without creating a vector in rust.
pub fn widget_slice<T: Widget>(mut widget: T) -> Vec<(Rect, Primative)>
//This is why associated types are garbage.
where
    // T::Layout: Widget,
    T::Layout: Widget + std::fmt::Debug,
{
    widget
        .as_uniform_layout_type()
        .into_iter()
        .map(|widget| (widget.area(), widget.primative()))
        .collect()
}

#[macro_export]
macro_rules! flex_center_4 {
    ($($widget:expr),*$(,)?) => {
        let widgets = [
            $(
                widget_slice($widget),
            )*
        ].concat();

        flex(&widgets)
    };
}

#[macro_export]
macro_rules! flex_center_3 {
    ($($widget:expr),*$(,)?) => {
        let widgets = [$(
            widget($widget),
        )*];

        flex(&widgets);
    };
}

#[macro_export]
macro_rules! flex_center_2 {
    ($($widget:expr),*$(,)?) => {
        let mut segments: Vec<Segment> = Vec::new();
        let viewport_width = ctx().area.width;
        let viewport_height = ctx().area.height;
        let mut total_width = 0;
        let mut max_width = 0;

        //The total height of largest widget in each segment.
        let mut total_height_of_largest = 0;
        let mut total_hsegments = 0;

        let mut max_height = 0;
        let mut horizontal_wrap = 0;
        let mut vertical_wrap = 0;

        let mut widgets = Vec::new();
        let count = $crate::count_expr!($($widget),*);
        let mut i = 0;

        $(
            i += 1;
            let mut widget = $widget;

            let area = widget.area();

            //Skip the zero width segment.
            //This is pretty much a hack and should be removed in the third re-write.
            if total_width + area.width > viewport_width && !(total_width == 0 || max_width == 0){
                segments.push(Segment {
                    direction: Direction::Horizontal,
                    size: total_width,
                    max: max_width,
                    widgets: core::mem::take(&mut widgets),
                });

                total_hsegments += 1;
                total_height_of_largest += max_height;
                max_height = 0;
                total_width = 0;
                max_width = 0;
            }

            total_width += area.width;
            // total_height += area.height;

            if area.width > max_width {
                max_width = area.width;
            }

            if area.height > max_height {
                max_height = area.height;
            }

            widgets.push((area, widget.primative()));

            //Don't like this part.
            if (i == count) {
                total_hsegments += 1;
                total_height_of_largest += max_height;
                segments.push(Segment {
                    direction: Direction::Horizontal,
                    size: total_width,
                    max: max_width,
                    widgets: core::mem::take(&mut widgets),
                })
            }
        )*

        let mut vspacing = viewport_height.saturating_sub(total_height_of_largest) / (total_hsegments + 1);
        let mut y = vspacing;

        for segment in segments {
            let mut spacing = viewport_width.saturating_sub(segment.size) / (segment.widgets.len()  + 1);
            let mut x = spacing;
            let mut max_height = 0;

            match segment.direction {
                Direction::Horizontal => {
                    for (mut area, primative) in segment.widgets {
                        if area.height > max_height {
                            max_height = area.height;
                        }

                        area.x = x;
                        area.y = y;
                        unsafe { COMMAND_QUEUE.push(Command {area, primative}) };
                        x += spacing + area.width;
                    }
                    y += max_height + vspacing;
                }
                Direction::Vertical => {
                }
            }
        }
    };
}

//This allows for evenly spaced horizontal centering.
#[macro_export]
macro_rules! flex_center {
    ($name:ident:$wrap:expr,$name2:ident:$padding:expr, $($widget:expr),*$(,)?) => {
        // let mut state = FlexState {};
        let count = $crate::count_expr!($($widget),*);
        let viewport_width = ctx().width();
        let viewport_height = ctx().height();
        let mut tallest_widget = 0;
        let mut metrics: Vec<WrapMetrics> = vec![WrapMetrics::default(); count];
        let mut current_metrics = 0;
        let wrap: bool = $wrap;

        $(
            let area = $widget.area().unwrap().clone();
            let next_total_width = metrics[current_metrics].total_width + area.width as usize;

            //Widget will wrap.
            if next_total_width > viewport_width && wrap {
                current_metrics += 1;
                metrics[current_metrics].total_width = area.width as usize;
                metrics[current_metrics].total_widgets = 1;
            } else {
                metrics[current_metrics].total_width = next_total_width;
                metrics[current_metrics].total_widgets += 1;
            }

            if area.height as usize > tallest_widget {
                tallest_widget = area.height as usize;
            }
        )*

        let mut x = 0;
        let mut y = 0;
        let mut gaps = 0;
        let mut i = 0;
        current_metrics = 0;
        let mut current_total = metrics[current_metrics].total_widgets;
        let mut total_width = metrics[current_metrics].total_width;
        let mut spacing = viewport_width.saturating_sub(total_width) / (current_total + 1);
        //TODO: Should be spacing + flex.area.x
        //currently no support for post fix on this macro.
        //I'll add the closure junk later.
        let mut x = spacing;

        //TODO: Vertical wrapping
        //called `Result::unwrap()` on an `Err` value: "Canvas height is 596, cannot draw at 600 (300y + 300h)"
        let padding: i32 = $padding;

        $(
            if i > current_total {
                current_metrics += 1;
                current_total = metrics[current_metrics].total_widgets;
                total_width = metrics[current_metrics].total_width;
                spacing = viewport_width.saturating_sub(total_width) / (current_total + 1);
                x = spacing;
            }

            let ptr = unsafe { $widget.as_mut_ptr() };
            // flex(ptr, &mut state);
            let widget = unsafe { ptr.as_mut().unwrap() };
            widget.calculate_area();
            let area = widget.area().unwrap();
            area.x = x as i32;
            x += spacing + area.width as usize;

            //The widget is overflowing the viewport
            //If the users wants to wrap move the x co-ordinate
            //back to the inital spacing position and increase
            //the y co-ordinate. The amount would be determined by
            //either the tallest widget or the vertical spacing
            //when centering both horizontally and vertically.
            if (area.x + area.width > viewport_width as i32) && wrap {
                    x = spacing;
                    //Assume their is no vertical centering.
                    y += tallest_widget;
                    area.x = x as i32;
                    area.y = y as i32 + padding;
            }

            if let Some(command) = widget.draw_command() {
                widget.try_click();
                unsafe { COMMAND_QUEUE.push(command) };
            }

            i += 1;
        )*
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    //cargo test --package softui --lib -- layout::tests::center --exact --show-output
    #[test]
    fn center() {
        let ctx = create_ctx("Softui", 800, 800);
        while ctx.event() != Some(Event::Quit) {
            ctx.fill(Color::BLACK);

            // flex_center!(
            //     rect().wh(10),
            //     rect().wh(17),
            //     rect().wh(20),
            //     rect().wh(232),
            //     rect().wh(10),
            //     rect().wh(32)
            // );

            //The third widget should wrap here.
            // flex_center!(wrap: true, rect().wh(300), rect().wh(300), rect().wh(300));

            //The last widget doesn't wrap here at all...
            // flex_center!(wrap: true, padding: 20, rect().w(500).h(100), rect().w(500).h(100), rect().w(500).h(100));

            flex_center_2!(
                rect().w(500).h(100),
                rect().w(500).h(100),
                rect().w(500).h(100)
            );

            ctx.draw_frame();
        }
    }
}
