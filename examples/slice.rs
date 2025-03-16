#![allow(unused)]
use softui::*;

pub trait Test {
    type Item;
    fn test(&self) -> &[Self::Item];
}

#[derive(Debug)]
pub struct Empty {}

impl Test for Empty {
    type Item = Empty;

    fn test(&self) -> &[Self::Item] {
        core::slice::from_ref(self)
    }
}

impl<T: Test, const N: usize> Test for &mut [T; N] {
    type Item = T;

    fn test(&self) -> &[Self::Item] {
        core::slice::from_ref(&self[0])
    }
}

fn run<T: Widget + std::fmt::Debug>(t: &mut T)
where
    T::Layout: Widget + std::fmt::Debug,
{
    let t = t.as_uniform_layout_type();
    for w in t {
        dbg!(std::any::type_name_of_val(w));
        dbg!(w.area());
    }
}

fn uniform<T: Widget>(slice: &[T]) {
    dbg!(slice.len());
}

fn main() {
    // let mut_array = &mut [Empty {}, Empty {}];

    // let group = std::iter::once(&Empty {});
    // let group = std::iter::once(mut_array);

    // for w in group {
    //     dbg!(w);
    // }

    //
    let _ctx = create_ctx("Softui", 800, 600);

    let width = 800;
    let mut test = text("this is a test").on_click(Left, |_| println!("{}", width));
    let mut test = text("abc");

    let mut slice = std::slice::from_mut(&mut test);
    let mut array = [rect(), rect()];
    let mut vec = vec![rect(), rect()];

    let m = &mut array;
    let u = m.as_uniform_layout_type();
    dbg!(std::any::type_name_of_val(u));

    run(&mut rect());
    run(&mut slice);
    run(&mut array);
    run(&mut vec);
}
