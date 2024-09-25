use super::{clicked, ctx, Command, MouseButton, Widget};
use std::marker::PhantomData;

impl<T: Widget, F: FnMut(&mut T)> Widget for Click<T, F> {
    #[inline]
    fn area(&mut self) -> Option<&mut super::Rect> {
        self.widget.area()
    }

    #[inline]
    fn layout_area(&mut self) -> Option<&mut super::Rect> {
        self.widget.layout_area()
    }

    #[inline]
    fn try_click(&mut self) {
        //TODO: Thread safety
        if clicked(ctx(), &mut self.widget, self.button) {
            (self.click_fn)(&mut self.widget);
        }
    }

    #[inline]
    fn draw_command(&self) -> Option<Command> {
        self.widget.draw_command()
    }
}

pub struct Click<T: Widget, F: FnMut(&mut T)> {
    pub widget: T,
    pub click_fn: F,
    pub button: MouseButton,
}

impl<T: Widget, F: FnMut(&mut T)> std::fmt::Debug for Click<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Click")
            .field("widget", &self.widget)
            // .field("click_fn", &self.click_fn)
            .field("button", &self.button)
            .finish()
    }
}

// pub struct Click<T: Widget, F: ClickContainer<T>> {
//     pub widget: T,
//     pub click_fn: F,
//     pub button: MouseButton,
// }

// pub trait ClickContainer<T> {
//     fn call(&mut self, widget: &mut T)
//     where
//         Self: Sized;
// }

// pub struct ClickContainerImpl<F, T> {
//     pub f: Option<F>,
//     pub _marker: PhantomData<T>,
// }

// impl<F, T> ClickContainer<T> for ClickContainerImpl<F, T>
// where
//     F: FnMut(&mut T),
// {
//     fn call(&mut self, widget: &mut T) {
//         if let Some(f) = &mut self.f {
//             f(widget);
//         }
//     }
// }
