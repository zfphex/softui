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

// pub struct Click<T: Widget, C: ClickContainer<T>> {
//     pub widget: T,
//     pub click_fn: C,
//     pub button: MouseButton,
//     pub click_fn_vec: Vec<(MouseButton, C)>,
// }

// impl<T: Widget, F: FnMut(&mut T)> Click<T, ClickContainerImpl<F, T>> {
//     pub fn on_click(&mut self, button: MouseButton, f: F) {
//         self.click_fn_vec.push((
//             button,
//             ClickContainerImpl {
//                 click_fn: Some(f),
//                 _marker: PhantomData::default(),
//             },
//         ));
//     }
// }

// impl<T: Widget, C: ClickContainer<T>> std::fmt::Debug for Click<T, C> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// impl<T: Widget, C: ClickContainer<T>> Widget for Click<T, C> {
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
//             self.click_fn.call(&mut self.widget);
//         }
//     }

//     #[inline]
//     fn draw_command(&self) -> Option<Command> {
//         self.widget.draw_command()
//     }
// }

// pub trait ClickContainer<T> {
//     fn call(&mut self, widget: &mut T)
//     where
//         Self: Sized;
// }

// pub struct ClickContainerImpl<F, T> {
//     pub click_fn: Option<F>,
//     pub _marker: PhantomData<T>,
// }

// impl<F, T> ClickContainer<T> for ClickContainerImpl<F, T>
// where
//     F: FnMut(&mut T),
// {
//     fn call(&mut self, widget: &mut T) {
//         if let Some(f) = &mut self.click_fn {
//             f(widget);
//         }
//     }
// }

pub trait Tuple<T: Widget> {
    fn call(&mut self, widget: &mut T);
}

pub struct ClickTuple<T: Widget, Tu: Tuple<T>> {
    pub widget: T,
    pub click: Tu,
}

pub trait OnClick {
    type OnClickFn;
    fn on_click(self, button: MouseButton, f: Self::OnClickFn) -> Self;
}

// impl<T: Widget, Tu: Tuple<T>> OnClick for ClickTuple<T, Tu> {
//     fn on_click(self) -> Self {
//         todo!()
//     }
// }

// impl<T: Widget, Tu: Tuple<T>, F: FnMut(&mut T)> OnClick for ClickTuple<T, (Tu, (MouseButton, F))>
// where
//     (Tu, (MouseButton, F)): Tuple<T>,
// {
//     type OnClickFn = F;
//     fn on_click(self, button: MouseButton, f: Self::OnClickFn) -> Self {
//         Self {
//             widget: self.widget,
//             click: (self.click, (button, f)),
//         }
//     }
// }

// impl<T: Widget, Tu: Tuple<T>> ClickTuple<T, Tu> {
//     pub fn on_click<F: FnMut(&mut T)>(
//         self,
//         button: MouseButton,
//         f: F,
//     ) -> ClickTuple<T, (Tu, (MouseButton, F))>
//     where
//         (Tu, (MouseButton, F)): Tuple<T>,
//     {
//         ClickTuple {
//             widget: self.widget,
//             click: (self.click, (button, f)),
//         }
//     }
// }

impl<T: Widget, Tu: Tuple<T>> std::fmt::Debug for ClickTuple<T, Tu> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClickTuple")
            .field("widget", &self.widget)
            // .field("click", &self.click)
            .finish()
    }
}

impl<T: Widget, Tu: Tuple<T>> Widget for ClickTuple<T, Tu> {
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
        self.click.call(&mut self.widget);
    }

    #[inline]
    fn draw_command(&self) -> Option<Command> {
        self.widget.draw_command()
    }
}

// pub struct ClickEvent<T: Widget, F: FnMut(&mut T)> {
//     pub f: F,
//     pub button: MouseButton,
//     pub _marker: PhantomData<T>,
// }

// impl<T: Widget, F: FnMut(&mut T)> Tuple<T> for ClickEvent<T, F> {
//     fn call(&mut self, widget: &mut T) {
//         (self.f)(widget);
//     }
// }

impl<T: Widget, F: FnMut(&mut T)> Tuple<T> for (MouseButton, F) {
    fn call(&mut self, widget: &mut T) {
        if clicked(ctx(), widget, self.0) {
            self.1(widget);
        }
    }
}

macro_rules! impl_click {
    ($len: tt; $($t:ident),*; $($idx:tt),*) => {
        impl<T: Widget, $($t: FnMut(&mut T)),*> Tuple<T> for  ($((MouseButton, $t, )),*)  {
            fn call(&mut self, widget: &mut T) {
                //TODO: Thread safet
                let ctx = ctx();
                $(
                    if clicked(ctx, widget, self.$idx.0) {
                        self.$idx.1(widget);
                    }
                )*
            }
        }
    };
}

//Left, Right, Middle, Mouse4, Mouse5 (5 total)
// impl_click!(1; F0; 0);
impl_click!(2; F0, F1; 0, 1);
impl_click!(3; F0, F1, F2; 0, 1, 2);
impl_click!(4; F0, F1, F2, F3; 0, 1, 2, 3);
impl_click!(5; F0, F1, F2, F3, F4; 0, 1, 2, 3, 4);
// impl_click!(6; F0, F1, F2, F3, F4, F5; 0, 1, 2, 3, 4, 5);
// impl_click!(7; F0, F1, F2, F3, F4, F5, F6; 0, 1, 2, 3, 4, 5, 6);
