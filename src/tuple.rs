use crate::*;

pub trait Tuple {
    //error: constant expression depends on a generic parameter
    // const LEN: usize;
    // fn array(&mut self) -> [&mut dyn View; Self::LEN];

    fn for_each<F: FnMut(&dyn Widget)>(&self, f: F);
    fn for_each_mut<F: FnMut(&mut dyn Widget)>(&mut self, f: &mut F);
    fn for_each_rev_mut<F: FnMut(&mut dyn Widget)>(&mut self, f: &mut F);
    fn len(&self) -> usize;
    fn first(&mut self) -> &mut dyn Widget;
}

//https://github.com/audulus/rui/blob/main/src/Tuple.rs
macro_rules! impl_view_tuple {
    ($n: tt; $($t:ident),*; $($s:tt),*; $($s_rev:tt),*) => {
        impl< $( $t: Widget, )* > Tuple for ( $( $t, )* ) {
            // const LEN: usize = $n;

            // fn array(&mut self) -> [&mut dyn View; Self::LEN] {
            //     [$(&mut self.$s,)*]
            // }

            fn for_each<F: FnMut(&dyn Widget)>(&self, mut f: F) {
                $( f(&self.$s); )*
            }
            fn for_each_mut<F: FnMut(&mut dyn Widget)>(&mut self, f: &mut F) {
                $( f(&mut self.$s); )*
            }
            fn for_each_rev_mut<F: FnMut(&mut dyn Widget)>(&mut self, f: &mut F) {
                $( f(&mut self.$s_rev); )*
            }
            fn len(&self) -> usize {
                $n
            }
            fn first(&mut self) -> &mut dyn Widget {
                &mut self.0 as &mut dyn Widget
            }
        }
    }
}

impl<V: Widget> Tuple for V {
    fn for_each<F: FnMut(&dyn Widget)>(&self, mut f: F) {
        f(self);
    }
    fn for_each_mut<F: FnMut(&mut dyn Widget)>(&mut self, f: &mut F) {
        f(self);
    }
    fn for_each_rev_mut<F: FnMut(&mut dyn Widget)>(&mut self, f: &mut F) {
        f(self);
    }
    fn len(&self) -> usize {
        1
    }
    fn first(&mut self) -> &mut dyn Widget {
        self as &mut dyn Widget
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
