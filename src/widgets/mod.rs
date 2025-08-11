use std::{ops::Deref, slice::Iter};

pub mod rectangle;
pub use rectangle::*;

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "svg")]
pub use svg::*;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "image")]
pub use image::*;

pub mod text;
pub use text::*;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub mod dwrite;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub use dwrite::*;

use crate::*;

pub trait Widget<'a>: std::fmt::Debug {
    fn size(&self) -> (usize, usize);

    // #[track_caller]
    // fn desired_size(&self) -> (Unit, Unit) {
    //     unimplemented!()
    // }
    fn desired_size(&self) -> (Unit, Unit);

    //TODO: Move into struct Size
    fn size_new(&self) -> Size{
        unimplemented!()
    }

    fn layout_new(&mut self, current_size: Size, parent: Rect) {
        unimplemented!()
    }

    fn layout(&mut self, area: Rect);
    fn handle_event(&mut self, ctx: &mut Context) {}
    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>);
    fn style(&self) -> Option<Style> {
        None
    }
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    // I don't want to add an associated type to every single widget definition
    // if it can be avoided.

    // #[inline]
    // unsafe fn as_slice(&mut self) -> &[Self::Layout] {
    //     unsafe { core::mem::transmute(core::slice::from_ref(self)) }
    // }

    fn on_click<F>(self, button: MouseButton, handler: F) -> Click<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        Click::new(self.style(), self, button, MouseAction::Clicked, handler)
    }

    fn on_press<F>(self, button: MouseButton, handler: F) -> Click<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        Click::new(self.style(), self, button, MouseAction::Pressed, handler)
    }

    fn on_release<F>(self, button: MouseButton, handler: F) -> Click<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        Click::new(self.style(), self, button, MouseAction::Released, handler)
    }

    //TODO: Change sizing methods and area to use UnitRect.
    //size:  impl Into<Unit>
    fn wh(mut self, size: usize) -> Self
    where
        Self: Sized,
    {
        self.area_mut().width = size;
        self.area_mut().height = size;
        self
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

    fn h_fill(mut self) -> Self
    where
        Self: Sized,
    {
        self.area_mut_new().height = Unit::Auto;
        self
    }

    fn w_fill(mut self) -> Self
    where
        Self: Sized,
    {
        self.area_mut_new().width = Unit::Auto;
        self
    }

    fn bg(self, color: Color) -> StyledWidget<Self>
    where
        Self: Sized,
    {
        StyledWidget::new(self).bg(color)
    }

    fn fg(self, color: Color) -> StyledWidget<Self>
    where
        Self: Sized,
    {
        StyledWidget::new(self).fg(color)
    }

    fn wh_new(mut self, unit: impl Into<Unit> + Copy) -> Self
    where
        Self: Sized,
    {
        self.area_mut_new().height = unit.into();
        self.area_mut_new().width = unit.into();
        self
    }

    fn w_new(mut self, unit: impl Into<Unit> + Copy) -> Self
    where
        Self: Sized,
    {
        self.area_mut_new().width = unit.into();
        self
    }

    fn h_new(mut self, unit: impl Into<Unit> + Copy) -> Self
    where
        Self: Sized,
    {
        self.area_mut_new().height = unit.into();
        self
    }

    fn area_mut(&mut self) -> &mut Rect;

    fn area_mut_new(&mut self) -> &mut UnitRect {
        todo!()
    }

    //TODO: Not sure if this is good since not all types will implement this.
    //It helps when chaing clicks or styles so I guess it is what it is.
    fn gap(self, gap: usize) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn margin(self, margin: usize) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn direction(self, direction: FlexDirection) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn padding(self, padding: usize) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

//TODO: Bring back uniform type layout in the macro layout macro.
//A collection of widgets does not have a size since it's just a random group of items.
//
impl<'a, T> Widget<'a> for Vec<T>
where
    T: Widget<'a>,
{
    fn size(&self) -> (usize, usize) {
        todo!()
    }

    fn layout(&mut self, area: Rect) {
        todo!()
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        todo!()
    }

    fn area_mut(&mut self) -> &mut Rect {
        todo!()
    }

    fn desired_size(&self) -> (Unit, Unit) {
        todo!()
    }
}

impl<'a, T> Widget<'a> for &'a mut [T]
where
    T: Widget<'a>,
{
    fn size(&self) -> (usize, usize) {
        todo!()
    }

    fn layout(&mut self, area: Rect) {
        todo!()
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        todo!()
    }

    fn area_mut(&mut self) -> &mut Rect {
        todo!()
    }

    fn desired_size(&self) -> (Unit, Unit) {
        todo!()
    }
}

impl<'a, T> Widget<'a> for &'a mut T
where
    T: Widget<'a>,
{
    fn size(&self) -> (usize, usize) {
        // `self` here is `&&'a mut T`, so we dereference twice to get to T.
        (**self).size()
    }

    fn layout(&mut self, area: Rect) {
        // `self` here is `&mut &'a mut T`, so we dereference once to get to &mut T.
        (*self).layout(area)
    }

    fn handle_event(&mut self, ctx: &mut Context) {
        (*self).handle_event(ctx)
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        (**self).draw(commands, style)
    }

    fn area_mut(&mut self) -> &mut Rect {
        (*self).area_mut()
    }

    fn desired_size(&self) -> (Unit, Unit) {
        todo!()
    }
}

// Unsafe: This implementation allows passing raw pointers to bypass the borrow checker's
// static analysis inside a loop. This is only safe if the user guarantees that the
// pointer remains valid for the entire duration of the frame's processing.
impl<'a, T> Widget<'a> for *mut T
where
    T: Widget<'a>,
{
    fn size(&self) -> (usize, usize) {
        // It is safe to create a shared reference from the raw pointer for reading.
        unsafe { (*self).as_ref().unwrap().size() }
    }

    fn layout(&mut self, area: Rect) {
        // It is safe to create a mutable reference here because this `&mut self`
        // guarantees we have exclusive access for this scope.
        unsafe { (*self).as_mut().unwrap().layout(area) }
    }

    fn handle_event(&mut self, ctx: &mut Context) {
        unsafe { (*self).as_mut().unwrap().handle_event(ctx) }
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        unsafe { (*self).as_ref().unwrap().draw(commands, style) }
    }

    fn area_mut(&mut self) -> &mut Rect {
        unsafe { (*self).as_mut().unwrap().area_mut() }
    }

    fn desired_size(&self) -> (Unit, Unit) {
        todo!()
    }
}
