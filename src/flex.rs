#![allow(unused)]
use crate::*;

#[derive(Default, Debug, Clone, Copy)]
pub enum FlexDirection {
    #[default]
    LeftRight,
    RightLeft,
    TopBottom,
    BottomTop,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Padding {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

impl From<(usize, usize, usize, usize)> for Padding {
    fn from(value: (usize, usize, usize, usize)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl Padding {
    pub const fn new(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

// Positioning Step
// +---------------------------------+
// | +-------------+ +-------------+ |
// | |             | |             | |
// | |             | |             | |
// | |             | +-------------+ |
// | |             |                 |
// | +-------------+                 |
// +------------------------+--------+

//flex!(
//    h!(rect().wh(300), rect().w(300).h(200)).gap(32)
//).padding(32)

//If no x and y values are specified start at the top left (0, 0)
//Assuming left to right layout.
//Start the x value at the left padding value.
//In this case our x value is `offset = 32`
//Our first widget is a container.
//We need to calculate the positioning of each of the widgets.
//The first will be at x = 32,
//the second will be at x = 32 + widget.width + gap
//The parent size will be padding.left + container.width + padding.right

//Because of the way macros work, the flex macro will need to also do sizing
//I can't chain together v!() and h!() without something delimiting the root node.
//There might be a way but I've been working on this for too long it's time to LOCK IN.

pub fn calculate_offset(direction: FlexDirection, padding: Padding) -> usize {
    match direction {
        FlexDirection::LeftRight => padding.left,
        FlexDirection::RightLeft => padding.right,
        FlexDirection::TopBottom => padding.top,
        FlexDirection::BottomTop => padding.bottom,
    }
}

pub fn draw_widgets(
    commands: &mut Vec<Command>,
    root_direction: FlexDirection,
    container: &mut Container,
    x_offset: &mut usize,
    y_offset: &mut usize,
    padding: Padding,
    gap: usize,
) {
    let last_index = container.widgets.len().saturating_sub(1);
    //Add the gap between the two containers to next widget
    for (i, widget) in container.widgets.iter_mut().enumerate() {
        let mut area = widget.area.clone();
        // dbg!(container.direction);

        area.x = *x_offset;
        area.y = *y_offset;

        match container.direction {
            FlexDirection::LeftRight => area.y += padding.top,
            FlexDirection::RightLeft => todo!(),
            FlexDirection::TopBottom => area.x += padding.left,
            FlexDirection::BottomTop => todo!(),
        }

        // This will always fail since TypelessWidget doesn't implement try_click :(
        // widget.area = area;
        // widget.try_click();
        // widget.run_click(area);

        commands.push(Command {
            area,
            primative: widget.primative(),
        });

        //Add gap for every element except for the last.
        if i != last_index {
            match container.direction {
                FlexDirection::LeftRight => *x_offset += container.gap,
                FlexDirection::RightLeft => todo!(),
                FlexDirection::TopBottom => *y_offset += container.gap,
                FlexDirection::BottomTop => todo!(),
            }
        }

        match container.direction {
            FlexDirection::LeftRight => *x_offset += area.width,
            FlexDirection::RightLeft => todo!(),
            FlexDirection::TopBottom => *y_offset += area.height,
            FlexDirection::BottomTop => todo!(),
        }
    }
}

/// Calculate the sizing of the flex container
/// This is the total size of all containers and widgets.
pub fn calculate_flex_sizing(direction: FlexDirection, gap: usize, container: &mut Container, area: &mut Rect) {
    match direction {
        FlexDirection::LeftRight => {
            area.width += container.area.width + gap;
            //Padding would have been overwritten here.
            area.height = area.height.max(container.area.height)
        }
        FlexDirection::RightLeft => todo!(),
        FlexDirection::TopBottom => {
            area.width = area.width.max(container.area.width);
            area.height += container.area.height + gap;
        }
        FlexDirection::BottomTop => todo!(),
    }
}

pub fn add_gap(direction: FlexDirection, x_offset: &mut usize, y_offset: &mut usize, gap: usize) {
    match direction {
        FlexDirection::LeftRight => *x_offset += gap,
        FlexDirection::RightLeft => todo!(),
        FlexDirection::TopBottom => *y_offset += gap,
        FlexDirection::BottomTop => todo!(),
    }
}

#[macro_export]
macro_rules! flex {
    //Assume eveything being pass in is a container.
    ($($container:expr),* $(,)?) => {{
        let f = |direction: $crate::FlexDirection, padding: $crate::Padding, gap: usize| {
            //Start at the left padding and move right for every widget.
            let mut x_offset = padding.left;
            //Start at the top padding and move down for every widget.
            let mut y_offset = padding.top;
            let mut area = Rect::default();
            let mut commands = Vec::new();
            let count = $crate::count_expr!($($container),*);

            $(
                //TODO: There is an error here if a widget is passed in instead of a container.
                //This error message should be clearer.
                let mut container = $container.build();
                $crate::calculate_flex_sizing(direction, gap, &mut container, &mut area);
                $crate::draw_widgets(&mut commands, direction, &mut container,  &mut x_offset, &mut y_offset, padding, gap);
                $crate::add_gap(direction, &mut x_offset, &mut y_offset, gap);
            )*

            //Take away the extra gap added.
            match direction {
                FlexDirection::LeftRight => area.width -= gap,
                FlexDirection::RightLeft => todo!(),
                FlexDirection::TopBottom => area.height -= gap,
                FlexDirection::BottomTop => todo!(),
            }

            area.width += padding.left + padding.right;
            area.height += padding.top + padding.bottom;

            $crate::Flex { commands, area }
        };

        $crate::DeferFlex {
            f,
            direction: $crate::FlexDirection::LeftRight,
            padding: $crate::Padding::default(),
            gap: 0,
            bg: None,
        }
    }}
}

#[derive(Debug)]
pub struct Flex {
    pub commands: Vec<Command>,
    pub area: Rect,
}

//Maybe group into one struct????
//Could also convert into widget to simplify calling code.
pub struct DeferFlex<F: FnMut(FlexDirection, Padding, usize) -> Flex> {
    pub f: F,
    pub direction: FlexDirection,
    pub padding: Padding,
    pub gap: usize,
    pub bg: Option<Color>,
}

impl<F: FnMut(FlexDirection, Padding, usize) -> Flex> Drop for DeferFlex<F> {
    fn drop(&mut self) {
        self.draw();
    }
}

impl<F: FnMut(FlexDirection, Padding, usize) -> Flex> DeferFlex<F> {
    pub fn draw(&mut self) {
        let mut flex = self.build();

        //Draw the background.
        if let Some(bg) = self.bg {
            flex.area.x = 0;
            queue_command(flex.area, Primative::Ellipse(0, bg))
        };

        for cmd in flex.commands {
            cmd.queue();
        }
    }
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }
    pub fn left_pad(mut self, padding: usize) -> Self {
        self.padding.left = padding;
        self
    }
    pub fn right_pad(mut self, padding: usize) -> Self {
        self.padding.right = padding;
        self
    }
    pub fn top_pad(mut self, padding: usize) -> Self {
        self.padding.top = padding;
        self
    }
    pub fn bottom_pad(mut self, padding: usize) -> Self {
        self.padding.bottom = padding;
        self
    }
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = Padding::new(padding, padding, padding, padding);
        self
    }
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }
}

impl<F: FnMut(FlexDirection, Padding, usize) -> Flex> Defer for DeferFlex<F> {
    type T = Flex;
    fn build(&mut self) -> Self::T {
        (self.f)(self.direction, self.padding, self.gap)
    }
}

//Sizing -> Positioning -> Rendering

// +---------------------------------+
// | +-------------+ +-------------+ |
// | |             | |             | |
// | |             | |             | |
// | |             | +-------------+ |
// | |             |                 |
// | +-------------+                 |
// +------------------------+--------+

//flex!(
//    h!(rect().wh(300), rect().w(300).h(200)).gap(32)
//).padding(32)

//First let's start with the child container.
//h!(rect().wh(300), rect().w(300).h(200)).gap(32)
//We are not calculating position with this.
//Instead we want to know the size of the container
//We already know the size of each widget.

//For this example width = 300 + 32 + 300 = 632
//                 height = 300

//Now this should be passed as a widget onto the next macro.
//I think that the widget trait will need to be reworked.
//Widget.primative() should return &[Primative] not a single Primative.
//This is because widgets need to be able to hold multiple other widgets.
//We also have the Widget.is_container() function.

pub fn calculate_h<T: Widget>(w: &T, width: &mut usize, height: &mut usize) {
    let area = w.area();
    *height = area.height.max(*height);
    *width += area.width;
}

//Size the elements first and then set the positions later.
#[macro_export]
macro_rules! h {
    ($($widget:expr),* $(,)?) => {{
        //TODO: Padding is unused here.
        let f = |padding: Padding, gap: usize| {
            let count = $crate::count_expr!($($widget),*);
            let total_gap = (count as i32 - 1) as usize * gap;
            let mut width = total_gap;
            let mut height = 0;
            let mut widgets = Vec::new();

            $(
                let w = &mut $widget;

                //This forces the container callback to be run.
                //TODO: Improve this
                let _ = unsafe { w.as_slice() };
                let is_container = unsafe { w.is_container() };
                if is_container {
                    $crate::calculate_h(w, &mut width, &mut height);
                }

                for child in unsafe { w.as_slice() } {
                    let area = child.area();
                    if !is_container {
                        height = area.height.max(height);
                        width += area.width;
                    }

                    //Type is stripped from Click<T> here so calls
                    //to widget.try_click() will always fail.
                    widgets.push(TypelessWidget{ area, primative: child.primative()})
                }
            )*

            //If there is only one element the gap is not important.
            Container { direction: $crate::FlexDirection::LeftRight, widgets, area: Rect::new(0, 0, width, height), gap: if count > 1 { gap } else { 0 } }
        };

        //Defer the creation of the container so that the builder pattern
        //can be used to modifiy aspects of the container such as gap and padding.
        $crate::DeferContainer {
            f,
            padding: Padding::default(),
            gap: 0,
            container: Container::default(),
        }
    }};
}

pub fn calculate_v<T: Widget>(w: &T, width: &mut usize, height: &mut usize) {
    let area = w.area();
    *width = area.width.max(*width);
    *height += area.height;
}

//There might be some way to reduce code-reuse here, but it's kind of necessary to avoid massive unintended match statements.
#[macro_export]
macro_rules! v {
    ($($widget:expr),* $(,)?) => {{
        //TODO: Padding is unused here.
        let f = |padding: Padding, gap: usize| {
            let count = $crate::count_expr!($($widget),*);
            let total_gap = (count as i32 - 1) as usize * gap;
            let mut width = 0;
            let mut height = total_gap;
            let mut widgets = Vec::new();

            $(
                let w = &mut $widget;

                //This forces the container callback to be run.
                //TODO: Improve this
                let _ = unsafe { w.as_slice() };
                let is_container = unsafe { w.is_container() };
                if is_container {
                    $crate::calculate_v(w, &mut width, &mut height);
                }

                for child in unsafe { w.as_slice() } {
                    if !is_container {
                        $crate::calculate_v(child, &mut width, &mut height);
                    }
                    widgets.push(TypelessWidget{ area: child.area(), primative: child.primative()})
                }
            )*

            //If there is only one element the gap is not important.
            Container { direction: $crate::FlexDirection::TopBottom, widgets, area: Rect::new(0, 0, width, height), gap: if count > 1 { gap } else { 0 }}
        };

        //Defer the creation of the container so that the builder pattern
        //can be used to modifiy aspects of the container such as gap and padding.
        $crate::DeferContainer {
            f,
            padding: Padding::default(),
            gap: 0,
            container: Container::default(),
        }
    }};
}

#[derive(Default, Debug)]
pub struct Container {
    pub widgets: Vec<TypelessWidget>,
    pub direction: FlexDirection,
    pub area: Rect,
    pub gap: usize,
}

impl Widget for Container {
    type Layout = Self;

    fn primative(&self) -> Primative {
        unreachable!()
    }

    fn area(&self) -> Rect {
        self.area
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

#[derive(Debug)]
pub struct TypelessWidget {
    pub area: Rect,
    pub primative: Primative,
}

impl Widget for TypelessWidget {
    type Layout = Self;

    //TODO: Should this be &Primative?
    fn primative(&self) -> Primative {
        self.primative.clone()
    }

    fn area(&self) -> Rect {
        self.area
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

pub struct DeferContainer<F> {
    pub f: F,
    pub padding: Padding,
    pub gap: usize,
    pub container: Container,
}

impl<F> DeferContainer<F> {
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = Padding::new(padding, padding, padding, padding);
        self
    }
}

impl<F> Widget for DeferContainer<F>
where
    F: FnMut(Padding, usize) -> Container,
{
    type Layout = TypelessWidget;

    fn primative(&self) -> Primative {
        unreachable!()
    }

    unsafe fn is_container(&self) -> bool {
        true
    }

    #[track_caller]
    fn area(&self) -> Rect {
        //This is quite the scary bit of code
        self.container.area
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        unimplemented!()
        // self.container = self.build();
        // Some(&mut self.container.area)
    }

    unsafe fn as_slice(&mut self) -> &[Self::Layout] {
        self.container = self.build();
        &self.container.widgets
    }
}

impl<F> Defer for DeferContainer<F>
where
    F: FnMut(Padding, usize) -> Container,
{
    type T = Container;
    fn build(&mut self) -> Self::T {
        (self.f)(self.padding, self.gap)
    }
}

pub trait Defer {
    type T;
    fn build(&mut self) -> Self::T;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_two_rect() {
        let mut container = h!(rect().wh(300), rect().w(300).h(200)).gap(32).build();
        assert_eq!(container.area.width, 632);
        assert_eq!(container.area.height, 300);
        assert_eq!(container.widgets.len(), 2);

        let flex = flex!(h!(rect().wh(300), rect().w(300).h(200)).gap(32))
            .padding(32)
            .gap(32)
            .build();

        assert_eq!(flex.area.width, 300 + 32 + 300 + 32 + 32);
        assert_eq!(flex.area.height, 300 + 32 + 32);
    }

    #[test]
    fn nested_containers() {
        let mut r = rect().wh(20).bg(blue());
        let mut r2 = r.bg(red());

        let blue = h!(r, r).gap(5).build();
        assert_eq!(blue.area.width, 20 + 5 + 20);
        assert_eq!(blue.area.height, 20);

        let red = v!(r2, r2).gap(5).build();
        assert_eq!(red.area.width, 20);
        assert_eq!(red.area.height, 20 + 5 + 20);

        let f = flex!(h!(r, r).gap(5), v!(r2, r2).gap(5)).gap(5).bg(green()).build();

        assert_eq!(f.area.width, 20 + 5 + 20 + 5 + 20);
        assert_eq!(f.area.height, 20 + 5 + 20);
    }
}
