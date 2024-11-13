use super::{clicked, ctx, Command, MouseButton, Widget};

macro_rules! impl_click {
    ($struct: ident;  $($t:ident),*; $next:ident; $next_fn:ident; $($idx:tt),*) => {
        #[doc(hidden)]
        #[derive(Clone)]
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

        #[doc(hidden)]
        impl<T: Widget, $($t: FnMut(&mut T)),*>Widget for $struct<T, $($t),*> {

            #[inline]
            fn area(&self) -> $crate::Rect {
                self.widget.area()
            }

            #[inline]
            fn area_mut(&mut self) -> Option<&mut super::Rect> {
                self.widget.area_mut()
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
            fn primative(&self) -> $crate::Primative {
                self.widget.primative()
            }
        }

        impl<T: Widget, $($t: FnMut(&mut T)),*> std::fmt::Debug for $struct<T, $($t),*> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct))
                    // .field("widget", &self.widget)
                    // .field("click", &self.click)
                    .finish()
            }
        }
    };
}

// TODO: Sealed widget trait might hide these Click structs from v! and h! errors.
// TODO: What about scroll up and down?
// TODO: What about mouse up and mouse down events...
impl_click!(Click0; F0; Click1; F1; 0);
impl_click!(Click1; F0, F1; Click2; F2; 0, 1);
impl_click!(Click2; F0, F1, F2; Click3; F3; 0, 1, 2);
impl_click!(Click3; F0, F1, F2, F3; Click4; F4; 0, 1, 2, 3);
impl_click!(Click4; F0, F1, F2, F3, F4; Click5; F5; 0, 1, 2, 3, 4);
// impl_click!(Click5; F0, F1, F2, F3, F4, F5; Click6; F6; 0, 1, 2, 3, 4, 5);

#[doc(hidden)]
#[derive(Clone)]
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
    pub fn on_click<F6: FnMut(&mut Self)>(self, _button: MouseButton, _f: F6) {
        unimplemented!(
            "On click chains are only support up to 5 times. Left, Right, Middle, Forward, Back."
        );
    }
}

// impl<
//         T: Widget,
//         F0: FnMut(&mut T),
//         F1: FnMut(&mut T),
//         F2: FnMut(&mut T),
//         F3: FnMut(&mut T),
//         F4: FnMut(&mut T),
//         F5: FnMut(&mut T),
//     > IntoVec for Click5<T, F0, F1, F2, F3, F4, F5>
// {
//     type T = Click5<T, F0, F1, F2, F3, F4, F5>;

//     fn into_vec(self) -> Vec<Self::T> {
//         vec![self]
//     }
// }
