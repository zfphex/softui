//! New layout system starting with something akin to flexbox.
//!

//There should be two types of flex, one for vertical and one for horizontal

use crate::{Direction, Widget};

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

pub fn flex<T: Widget>(widget: *mut T, state: &mut FlexState) {
    let widget = unsafe { widget.as_mut().unwrap() };
}

pub struct FlexState {}

#[derive(Debug, Clone, Default)]
pub struct WrapMetrics {
    pub total_width: usize,
    pub total_widgets: usize,
}

//This allows for evenly spaced horizontal centering.
#[macro_export]
macro_rules! flex_center {
    ($($widget:expr),*$(,)?) => {
        let mut state = FlexState {};
        let count = $crate::count_expr!($($widget),*);
        let viewport_width = ctx().width();
        let mut tallest_widget = 0;
        let mut metrics: Vec<WrapMetrics> = vec![WrapMetrics::default(); count];
        let mut current_metrics = 0;

        $(
            let area = $widget.area().unwrap().clone();
            let next_total_width = metrics[current_metrics].total_width + area.width as usize;

            //Widget will wrap.
            if next_total_width > viewport_width {
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

        let wrap = true;
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
                area.y = y as i32;
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
            flex_center!(rect().wh(300), rect().wh(300), rect().wh(300));

            ctx.draw_frame();
        }
    }
}
