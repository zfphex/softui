use softui::*;

fn run<T: Widget>(t: &mut T)
where
    <T as softui::Widget>::Layout: std::fmt::Debug,
{
    let test = t.as_uniform_layout_type();
    dbg!(test);
}

fn uniform<T: Widget>(slice: &[T]) {
    dbg!(slice.len());
}

fn main() {
    let _ctx = create_ctx("Softui", 800, 600);

    // let width = 800;
    // let mut test = text("this is a test").on_click(Left, |_| println!("{}", width));
    let mut test = text("abc");

    let mut slice = std::slice::from_mut(&mut test);
    let mut array = [rect(), rect()];
    let mut vec = vec![rect(), rect()];

    run(&mut rect());
    run(&mut [rect(), rect()]);
    run(&mut slice);
    run(&mut array);
    run(&mut vec);

    uniform((&mut rect()).as_uniform_layout_type());
    uniform((&mut [rect(), rect()]).as_uniform_layout_type());
    uniform((&mut slice).as_uniform_layout_type());
    uniform((&mut array).as_uniform_layout_type());
    uniform((&mut vec).as_uniform_layout_type());
}
