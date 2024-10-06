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

// pub trait Tuple<T: Widget> {
//     fn call(&mut self, widget: &mut T);
// }

// pub struct ClickTuple<T: Widget, Tu: Tuple<T>> {
//     pub widget: T,
//     pub click: Tu,
// }

// pub trait OnClick {
//     type OnClickFn;
//     fn on_click(self, button: MouseButton, f: Self::OnClickFn) -> Self;
// }

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

// impl<T: Widget, Tu: Tuple<T>> std::fmt::Debug for ClickTuple<T, Tu> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("ClickTuple")
//             .field("widget", &self.widget)
//             // .field("click", &self.click)
//             .finish()
//     }
// }

// impl<T: Widget, Tu: Tuple<T>> Widget for ClickTuple<T, Tu> {
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
//         self.click.call(&mut self.widget);
//     }

//     #[inline]
//     fn draw_command(&self) -> Option<Command> {
//         self.widget.draw_command()
//     }
// }

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

// impl<T: Widget, F: FnMut(&mut T)> Tuple<T> for (MouseButton, F) {
//     fn call(&mut self, widget: &mut T) {
//         if clicked(ctx(), widget, self.0) {
//             self.1(widget);
//         }
//     }
// }

// macro_rules! impl_click {
//     ($len: tt; $($t:ident),*; $($idx:tt),*) => {
//         impl<T: Widget, $($t: FnMut(&mut T)),*> Tuple<T> for  ($((MouseButton, $t, )),*)  {
//             fn call(&mut self, widget: &mut T) {
//                 //TODO: Thread safet
//                 let ctx = ctx();
//                 $(
//                     if clicked(ctx, widget, self.$idx.0) {
//                         self.$idx.1(widget);
//                     }
//                 )*
//             }
//         }
//     };
// }

//Left, Right, Middle, Mouse4, Mouse5 (5 total)
// impl_click!(1; F0; 0);

// impl_click!(2; F0, F1; 0, 1);
// impl_click!(3; F0, F1, F2; 0, 1, 2);
// impl_click!(4; F0, F1, F2, F3; 0, 1, 2, 3);
// impl_click!(5; F0, F1, F2, F3, F4; 0, 1, 2, 3, 4);

// impl_click!(6; F0, F1, F2, F3, F4, F5; 0, 1, 2, 3, 4, 5);
// impl_click!(7; F0, F1, F2, F3, F4, F5, F6; 0, 1, 2, 3, 4, 5, 6);

// pub trait ClickImpl {
//     fn call(self);
// }

macro_rules! impl_click {
    ($struct: ident;  $($t:ident),*; $next:ident; $next_fn:ident; $($idx:tt),*) => {
        pub struct $struct<T: Widget, $($t: FnMut(&mut T)),*> {
            pub widget: T,
            pub click: ($((MouseButton, $t)),*,),
        }

        impl<T: Widget, $($t: FnMut(&mut T)),*> $struct<T, $($t),*> {
            pub fn on_click<$next_fn: FnMut(&mut T)>(self, button: MouseButton, f: $next_fn) -> $next<T, $($t),*, $next_fn> {
                $next {
                    widget: self.widget,
                    click: ($(self.click.$idx),*, (button, f)),
                }
            }
        }

        impl<T: Widget, $($t: FnMut(&mut T)),*>Widget for $struct<T, $($t),*> {
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
                let ctx = ctx();
                $(
                    if clicked(ctx, &mut self.widget, self.click.$idx.0) {
                        self.click.$idx.1(&mut self.widget);
                    }
                )*
            }

            #[inline]
            fn draw_command(&self) -> Option<Command> {
                self.widget.draw_command()
            }
        }

        impl<T: Widget, $($t: FnMut(&mut T)),*> std::fmt::Debug for $struct<T, $($t),*> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Click0")
                    .field("widget", &self.widget)
                    // .field("click", &self.click)
                    .finish()
            }
        }
    };
}

impl_click!(Click0; F0; Click1; F1; 0);
impl_click!(Click1; F0, F1; Click2; F2; 0, 1);
impl_click!(Click2; F0, F1, F2; Click3; F3; 0, 1, 2);
impl_click!(Click3; F0, F1, F2, F3; Click4; F4; 0, 1, 2, 3);
impl_click!(Click4; F0, F1, F2, F3, F4; Click5; F5; 0, 1, 2, 3, 4);

pub struct Click5<
    T: Widget,
    F0: FnMut(&mut T),
    F1: FnMut(&mut T),
    F2: FnMut(&mut T),
    F3: FnMut(&mut T),
    F4: FnMut(&mut T),
    F5: FnMut(&mut T),
> {
    pub widget: T,
    pub click: (
        (MouseButton, F0),
        (MouseButton, F1),
        (MouseButton, F2),
        (MouseButton, F3),
        (MouseButton, F4),
        (MouseButton, F5),
    ),
}

impl<
        T: Widget,
        F0: FnMut(&mut T),
        F1: FnMut(&mut T),
        F2: FnMut(&mut T),
        F3: FnMut(&mut T),
        F4: FnMut(&mut T),
        F5: FnMut(&mut T),
    > Click5<T, F0, F1, F2, F3, F4, F5>
{
    pub fn on_click<F6: FnMut(&mut Self)>(self, button: MouseButton, f: F6) {
        unimplemented!(
            "On click chains are only support up to 5 times. Left, Right, Middle, Forward, Back."
        );
    }
}

// impl_click!(Click5; F0, F1, F2, F3, F4, F5; Click6; F6; 0, 1, 2, 3, 4, 5);

// pub struct Click0<T: Widget, F0: FnMut(&mut T)> {
//     pub widget: T,
//     pub click: ((MouseButton, F0),),
// }

// pub struct Click1<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T)> {
//     pub widget: T,
//     pub click: ((MouseButton, F0), (MouseButton, F1)),
// }

// pub struct Click2<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T), F2: FnMut(&mut T)> {
//     pub widget: T,
//     pub click: ((MouseButton, F0), (MouseButton, F1), (MouseButton, F2)),
// }

// impl<T: Widget, F0: FnMut(&mut T)> Click0<T, F0> {
//     pub fn on_click<F1: FnMut(&mut T)>(self, button: MouseButton, f: F1) -> Click1<T, F0, F1> {
//         Click1 {
//             widget: self.widget,
//             click: (self.click.0, (button, f)),
//         }
//     }
// }

// impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T)> Click1<T, F0, F1> {
//     pub fn on_click<F2: FnMut(&mut T)>(self, button: MouseButton, f: F2) -> Click2<T, F0, F1, F2> {
//         Click2 {
//             widget: self.widget,
//             click: (self.click.0, self.click.1, (button, f)),
//         }
//     }
// }

// impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T)> ClickImpl for Click1<T, F0, F1> {
//     type Next = ();
//     type F = ();
//     fn call(mut self) {
//         self.click.0 .1(&mut self.widget);
//         self.click.1 .1(&mut self.widget);
//     }
//     fn on_click(self, button: MouseButton, f: F) -> Self::Next {
//         core::panicking::panic("not yet implemented")
//     }
// }
