use softui::*;

fn main() {
    let parent = size(0, 0, 500, 600);

    let r1 = rect().w(20.percent()).h(300);
    let r2 = rect().w(80.percent()).h(400);

    let h1 = h!(r1, r2);
    let h1_size = h1.size_new(parent);

    dbg!(h1_size);
}
