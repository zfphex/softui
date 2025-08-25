use crate::*;

#[test]
fn basic_multi() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new();
    group.children.push(Box::new(h!(
        rect().w(20.percent()).h(20).bg(blue()),
        rect().w(30.percent()).h(20).bg(blue())
    )));

    group
        .children
        .push(Box::new(h!(rect().w(50.percent()).h(20).bg(green()))));

    //TODO: We need to pass the widgets left to the next child container

    group.size(parent);

    assert_eq!(group.size.width, Unit::Auto(0));
    assert_eq!(group.size.height, Unit::Pixel(20));

    unsafe { RECURSE = 0 };
    println!();

    group.size(parent);

    assert_eq!(group.size.width, Unit::Pixel(800));
    assert_eq!(group.size.height, Unit::Pixel(600));

    let h1 = group.children[0].size_mut().clone();
    assert_eq!(h1.x, Unit::Pixel(0));
    assert_eq!(h1.y, Unit::Pixel(0));
    assert_eq!(h1.width, Unit::Pixel(400));
    assert_eq!(h1.height, Unit::Pixel(300));

    // let h1c = group.children[0].children();

    // let r1 = h1c[0].size_mut().clone();
    // assert_eq!(r1.x, Unit::Pixel(0));
    // assert_eq!(r1.y, Unit::Pixel(0));
    // assert_eq!(r1.width, Unit::Pixel(160));
    // assert_eq!(r1.height, Unit::Pixel(20));

    // let h2 = group.children[1].size_mut().clone();
    // assert_eq!(h2.x, Unit::Pixel(160));
    // assert_eq!(h2.y, Unit::Pixel(0));
    // assert_eq!(h2.width, Unit::Pixel(640));
    // assert_eq!(h2.height, Unit::Pixel(20));

    // let h2c = group.children[1].children();

    // let r2 = h2c[0].size_mut().clone();
    // assert_eq!(r2.x, Unit::Pixel(160));
    // assert_eq!(r2.y, Unit::Pixel(0));
    // assert_eq!(r2.width, Unit::Pixel(640));
    // assert_eq!(r2.height, Unit::Pixel(20));
}

#[test]
fn basic_position() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(LeftRight);
    group.children.push(Box::new(rect().w(20.percent()).h(20)));
    group.children.push(Box::new(rect().w(80.percent()).h(20)));

    group.size(parent);
    group.size(parent);

    group.position(parent);

    let r1 = group.children[0].size_mut().clone();
    assert_eq!(r1.x, Unit::Pixel(0));
    assert_eq!(r1.y, Unit::Pixel(0));

    let r2 = group.children[1].size_mut().clone();
    assert_eq!(r2.x, Unit::Pixel(160));
    assert_eq!(r2.y, Unit::Pixel(0));
}

#[test]
fn basic_size() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(LeftRight);
    group.children.push(Box::new(rect().w(20.percent()).h(20)));
    group.children.push(Box::new(rect().w(80.percent()).h(20)));

    group.size(parent);

    assert_eq!(group.size.width, Unit::Auto(0));
    assert_eq!(group.size.height, Unit::Pixel(20));
    assert_eq!(group.size.widgets_left, Some(2));

    unsafe { RECURSE = 0 };
    println!();

    group.size(parent);

    assert_eq!(group.size.width, Unit::Pixel(800));
    assert_eq!(group.size.height, Unit::Pixel(20));

    let v1 = group.children[0].size_mut().clone();

    assert_eq!(v1.width, Unit::Pixel(160));
    assert_eq!(v1.height, Unit::Pixel(20));

    let v2 = group.children[1].size_mut().clone();
    assert_eq!(v2.width, Unit::Pixel(640));
    assert_eq!(v2.height, Unit::Pixel(20));
}

#[test]
fn basic_nested_size() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(LeftRight);

    let mut subgroup = Group::new().direction(LeftRight);
    subgroup.children.push(Box::new(rect().w(20.percent()).h(20)));
    subgroup.children.push(Box::new(rect().w(80.percent()).h(20)));
    group.children.push(Box::new(subgroup));

    group.size(parent);

    assert_eq!(group.size.width, Unit::Auto(0));
    assert_eq!(group.size.height, Unit::Pixel(20));
    assert_eq!(group.size.widgets_left, Some(1));

    unsafe { RECURSE = 0 };
    println!();

    group.size(parent);

    assert_eq!(group.size.width, Unit::Pixel(800));
    assert_eq!(group.size.height, Unit::Pixel(20));

    let children = group.children[0].children();

    let v1 = children[0].size_mut().clone();

    assert_eq!(v1.width, Unit::Pixel(160));
    assert_eq!(v1.height, Unit::Pixel(20));

    let v2 = children[1].size_mut().clone();
    assert_eq!(v2.width, Unit::Pixel(640));
    assert_eq!(v2.height, Unit::Pixel(20));
}

#[test]
fn basic_fixed_size() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(LeftRight);

    group.children.push(Box::new(rect().wh(20)));

    group.children.push(Box::new(rect().w(20.percent()).h(20)));
    group.children.push(Box::new(rect().w(80.percent()).h(20)));

    group.size(parent);

    assert_eq!(group.size.width, Unit::Auto(20));
    assert_eq!(group.size.height, Unit::Pixel(20));
    assert_eq!(group.size.widgets_left, Some(2));

    // dbg!(&group.size);

    unsafe { RECURSE = 0 };
    println!();

    group.size(parent);

    assert_eq!(group.size.width, Unit::Pixel(800));
    assert_eq!(group.size.height, Unit::Pixel(20));

    let v1 = group.children[0].size_mut().clone();

    assert_eq!(v1.width, Unit::Pixel(20));
    assert_eq!(v1.height, Unit::Pixel(20));

    let v2 = group.children[1].size_mut().clone();
    assert_eq!(v2.width, Unit::Pixel(156));
    assert_eq!(v2.height, Unit::Pixel(20));

    let v3 = group.children[2].size_mut().clone();
    assert_eq!(v3.width, Unit::Pixel(624));
    assert_eq!(v3.height, Unit::Pixel(20));
}

#[test]
fn basic_nested_fixed_size() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(LeftRight);
    group.children.push(Box::new(rect().wh(20)));

    let mut subgroup = Group::new().direction(LeftRight);
    subgroup.children.push(Box::new(rect().w(20.percent()).h(20)));
    subgroup.children.push(Box::new(rect().w(80.percent()).h(20)));
    group.children.push(Box::new(subgroup));

    group.size(parent);

    assert_eq!(group.size.width, Unit::Auto(20));
    assert_eq!(group.size.height, Unit::Pixel(20));
    assert_eq!(group.size.widgets_left, Some(1));

    unsafe { RECURSE = 0 };
    println!();

    group.size(parent);

    assert_eq!(group.size.width, Unit::Pixel(800));
    assert_eq!(group.size.height, Unit::Pixel(20));

    let children = group.children[1].children();

    let v1 = children[0].size_mut().clone();

    assert_eq!(v1.width, Unit::Pixel(156));
    assert_eq!(v1.height, Unit::Pixel(20));

    let v2 = children[1].size_mut().clone();
    assert_eq!(v2.width, Unit::Pixel(624));
    assert_eq!(v2.height, Unit::Pixel(20));
}
