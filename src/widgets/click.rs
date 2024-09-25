use super::{clicked, ctx, Command, MouseButton, Widget};
use std::marker::PhantomData;

// pub struct Click<T: Widget, F: FnMut(&mut T)> {
//     pub widget: T,
//     pub click_fn: F,
//     pub button: MouseButton,
// }

// impl<T: Widget, F: FnMut(&mut T)> Click<T, F> {
//     pub fn on_click(&mut self, button: MouseButton, f: F) {}
// }

// impl<T: Widget, F: FnMut(&mut T)> Widget for Click<T, F> {
//     #[inline]
//     fn area(&mut self) -> Option<&mut super::Rect> {
//         self.widget.area()
//     }

//     #[inline]
//     fn layout_area(&mut self) -> Option<&mut super::Rect> {
//         self.widget.layout_area()
//     }

//     #[inline]
//     fn try_click(&mut self) {
//         //TODO: Thread safety
//         if clicked(ctx(), &mut self.widget, self.button) {
//             (self.click_fn)(&mut self.widget);
//         }
//     }

//     #[inline]
//     fn draw_command(&self) -> Option<Command> {
//         self.widget.draw_command()
//     }
// }

// impl<T: Widget, F: FnMut(&mut T)> std::fmt::Debug for Click<T, F> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Click")
//             .field("widget", &self.widget)
//             // .field("click_fn", &self.click_fn)
//             .field("button", &self.button)
//             .finish()
//     }
// }

pub struct Click<T: Widget, C: ClickContainer<T>> {
    pub widget: T,
    pub click_fn: C,
    pub button: MouseButton,
    pub click_fn_vec: Vec<(MouseButton, C)>,
}

impl<T: Widget, F: FnMut(&mut T)> Click<T, ClickContainerImpl<F, T>> {
    pub fn on_click(&mut self, button: MouseButton, f: F) {
        self.click_fn_vec.push((
            button,
            ClickContainerImpl {
                click_fn: Some(f),
                _marker: PhantomData::default(),
            },
        ));
    }
}

impl<T: Widget, C: ClickContainer<T>> std::fmt::Debug for Click<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Widget, C: ClickContainer<T>> Widget for Click<T, C> {
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
            self.click_fn.call(&mut self.widget);
        }
    }

    #[inline]
    fn draw_command(&self) -> Option<Command> {
        self.widget.draw_command()
    }
}

pub trait ClickContainer<T> {
    fn call(&mut self, widget: &mut T)
    where
        Self: Sized;
}

pub struct ClickContainerImpl<F, T> {
    pub click_fn: Option<F>,
    pub _marker: PhantomData<T>,
}

impl<F, T> ClickContainer<T> for ClickContainerImpl<F, T>
where
    F: FnMut(&mut T),
{
    fn call(&mut self, widget: &mut T) {
        if let Some(f) = &mut self.click_fn {
            f(widget);
        }
    }
}
