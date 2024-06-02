use crate::*;

pub trait View {
    fn area(&mut self) -> &mut Rect;
}

pub trait Tuple {
    fn for_each<F: FnMut(&mut dyn View)>(&mut self, f: &mut F);
    fn for_each_rev<F: FnMut(&mut dyn View)>(&mut self, f: &mut F);
    fn len(&self) -> usize;
    fn first(&mut self) -> &mut dyn View;
}

static mut VOID_RECT: Rect = Rect::new(0, 0, 0, 0);

impl View for () {
    fn area(&mut self) -> &mut Rect {
        unsafe { &mut VOID_RECT }
    }
}

//https://github.com/audulus/rui/blob/main/src/Tuple.rs
macro_rules! impl_view_tuple {
    ($n: tt; $($t:ident),*; $($s:tt),*; $($s_rev:tt),*) => {

        impl< $( $t: View, )* > Tuple for ( $( $t, )* ) {
            fn for_each<F: FnMut(&mut dyn View)>(&mut self, f: &mut F) {
                $( f(&mut self.$s); )*
            }
            fn for_each_rev<F: FnMut(&mut dyn View)>(&mut self, f: &mut F) {
                $( f(&mut self.$s_rev); )*
            }
            fn len(&self) -> usize {
                $n
            }
            fn first(&mut self) -> &mut dyn View {
                &mut self.0 as &mut dyn View
            }
        }
    }
}

//Limit tuples to 10 items long. Helps with compile times.
impl_view_tuple!(1; V0; 0; 0);
impl_view_tuple!(2; V0, V1; 0, 1; 1, 0);
impl_view_tuple!(3; V0, V1, V2; 0, 1, 2; 2, 1, 0);
impl_view_tuple!(4; V0, V1, V2, V3; 0, 1, 2, 3; 3, 2, 1, 0);
impl_view_tuple!(5; V0, V1, V2, V3, V4; 0, 1, 2, 3, 4; 4, 3, 2, 1, 0);
impl_view_tuple!(6; V0, V1, V2, V3, V4, V5; 0, 1, 2, 3, 4, 5; 5, 4, 3, 2, 1, 0);
impl_view_tuple!(7; V0, V1, V2, V3, V4, V5, V6; 0, 1, 2, 3, 4, 5, 6; 6, 5, 4, 3, 2, 1, 0);
impl_view_tuple!(8; V0, V1, V2, V3, V4, V5, V6, V7; 0, 1, 2, 3, 4, 5, 6, 7; 7, 6, 5, 4, 3, 2, 1, 0);
impl_view_tuple!(9; V0, V1, V2, V3, V4, V5, V6, V7, V8; 0, 1, 2, 3, 4, 5, 6, 7, 8; 8, 7, 6, 5, 4, 3, 2, 1, 0);
impl_view_tuple!(10; V0, V1, V2, V3, V4, V5, V6, V7, V8, V9; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9; 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);
