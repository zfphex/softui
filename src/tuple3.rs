use crate::*;

pub struct OnClick3<T: Widget> {
    widget: T,
    function: usize,
    button: MouseButton,
}

pub trait Tuple3 {
    fn for_each(&mut self, f: impl FnMut((&mut dyn Widget, usize)));
    fn len(&self) -> usize;
    fn first(&mut self) -> &mut dyn Widget;
    fn get(&self, index: usize) -> Option<&dyn Widget>;
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Widget>;
}

impl<V0: Widget> Tuple3 for OnClick3<V0> {
    fn for_each(&mut self, mut f: impl FnMut((&mut dyn Widget, usize))) {
        f((&mut self.widget as &mut dyn Widget, self.function))
    }
    fn len(&self) -> usize {
        1
    }
    fn first(&mut self) -> &mut dyn Widget {
        &mut self.widget as &mut dyn Widget
    }
    fn get(&self, index: usize) -> Option<&dyn Widget> {
        Some(&self.widget as &dyn Widget)
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Widget> {
        Some(&mut self.widget as &mut dyn Widget)
    }
}

// #[macro_export]
// macro_rules! impl_tuple {
//  ($len: tt; $($t:ident, $c:ident),*; $($idx:tt),*; $($idx_rev:tt),*) =>  {
//   impl< $( $t: Widget, $c: FnMut(&mut $t) -> () ,)* > Tuple2 for ( $( OnClickWrapper<$t, $c> ),*,  ){
//         //Call the on click function for every widget.
//         fn handle_on_click(&mut self) {
//             $(
//                 let wrapper = &mut self.$idx;
//                 if let Some((f, button)) = &mut wrapper.f {
//                     let w = &mut wrapper.widget as &mut dyn Widget;
//                     if clicked_dyn(ctx(), w, *button) {
//                         (f)(&mut wrapper.widget);
//                     }
//                 }
//             )*
//         }
//         fn for_each<F: FnMut(&dyn Widget)>(&self, mut f: F) {
//             $( f(&self.$idx.widget); )*
//         }
//         fn for_each_mut<F: FnMut(&mut dyn Widget)>(&mut self, f: &mut F) {
//             $( f(&mut self.$idx.widget); )*
//         }
//         fn for_each_rev_mut<F: FnMut(&mut dyn Widget)>(&mut self, f: &mut F) {
//             $( f(&mut self.$idx_rev.widget); )*
//         }
//         fn len(&self) -> usize {
//             $len
//         }
//         fn first(&mut self) -> &mut dyn Widget {
//             &mut self.0.widget as &mut dyn Widget
//         }
//         fn get(&self, index: usize) -> Option<&dyn Widget> {
//             match index {
//                 $($idx => Some(&self.$idx.widget as &dyn Widget),)*
//                 _ => unreachable!(),
//             }
//         }
//         fn get_mut(&mut self, index: usize) -> Option<&mut dyn Widget> {
//             match index {
//                 $($idx => Some(&mut self.$idx.widget as &mut dyn Widget),)*
//                 _ => None,
//             }
//         }
//     }
//  }
// }

// impl_tuple!(1; V0, C0; 0; 0);
// impl_tuple!(2; V0, C0, V1, C1; 0, 1; 1, 0);
// impl_tuple!(3; V0, C0, V1, C1, V2, C2; 0, 1, 2; 2, 1, 0);
// impl_tuple!(4; V0, C0, V1, C1, V2, C2, V3, C3; 0, 1, 2, 3; 3, 2, 1, 0);
// impl_tuple!(5; V0, C0, V1, C1, V2, C2, V3, C3, V4, C4; 0, 1, 2, 3, 4; 4, 3, 2, 1, 0);
// impl_tuple!(6; V0, C0, V1, C1, V2, C2, V3, C3, V4, C4, V5, C5; 0, 1, 2, 3, 4, 5; 5, 4, 3, 2, 1, 0);

// impl_tuple!(7; V0, V1, V2, V3, V4, V5, V6; 0, 1, 2, 3, 4, 5, 6; 6, 5, 4, 3, 2, 1, 0);
// impl_tuple!(8; V0, V1, V2, V3, V4, V5, V6, V7; 0, 1, 2, 3, 4, 5, 6, 7; 7, 6, 5, 4, 3, 2, 1, 0);
// impl_tuple!(9; V0, V1, V2, V3, V4, V5, V6, V7, V8; 0, 1, 2, 3, 4, 5, 6, 7, 8; 8, 7, 6, 5, 4, 3, 2, 1, 0);
// impl_tuple!(10; V0, V1, V2, V3, V4, V5, V6, V7, V8, V9; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9; 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);
