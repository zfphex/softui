use softui::*;

// Box<[T]> can go die tbh.
// fn run<'a, T: Widget + 'a, C: IntoSlice<'a, T> + ?Sized>(c: &'a mut C) {
fn run<'a, T: Widget + 'a, C: IntoSlice<'a, T>>(c: &'a mut C) {
    let slice = c.into_slice();
    for w in slice {
        dbg!(w.area_mut());
    }
}

pub trait IntoSlice<'a, T: Widget> {
    fn into_slice(&'a mut self) -> &'a mut [T];
}

impl<'a, T: Widget> IntoSlice<'a, T> for T {
    fn into_slice(&'a mut self) -> &'a mut [T] {
        std::slice::from_mut(self)
    }
}

impl<'a, T: Widget> IntoSlice<'a, T> for &'a mut [T] {
    fn into_slice(&'a mut self) -> &'a mut [T] {
        self
    }
}

impl<'a, T: Widget> IntoSlice<'a, T> for Vec<T> {
    fn into_slice(&'a mut self) -> &'a mut [T] {
        self.as_mut_slice()
    }
}

impl<'a, T: Widget, const N: usize> IntoSlice<'a, T> for [T; N] {
    fn into_slice(&'a mut self) -> &'a mut [T] {
        self
    }
}

// impl<'a, T: Widget, const N: usize> IntoSlice<'a, T> for Box<[T; N]> {
//     fn into_slice(&'a mut self) -> &'a mut [T] {
//         self.as_mut_slice()
//     }
// }

// impl<'a, T: Widget> IntoSlice<'a, T> for Box<[T]> {
//     fn into_slice(&'a mut self) -> &'a mut [T] {
//         self
//     }
// }

fn main() {
    let _ctx = create_ctx("Softui", 800, 600);
    let width = 800;

    let mut test = text("this is a test").on_click(Left, |_| println!("{}", width));

    let mut slice = std::slice::from_mut(&mut test);
    let mut array = [rect(), rect()];
    let mut vec = vec![rect(), rect()];
    // let mut b = Box::new([rect(), rect()]);

    run(&mut rect());
    run(&mut [rect(), rect()]);

    run(&mut slice);
    run(&mut array);
    // run(&mut vec);
}
