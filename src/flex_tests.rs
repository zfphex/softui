use crate::*;

#[test]
fn basic_size() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(LeftRight);
    group.children.push(Box::new(rect().w(20.percent()).h(20)));
    group.children.push(Box::new(rect().w(80.percent()).h(20)));

    group.size_new(parent);

    assert_eq!(group.size.width, Unit::Auto(0));
    assert_eq!(group.size.height, Unit::Pixel(20));
    assert_eq!(group.size.widgets_left, Some(2));

    unsafe { RECURSE = 0 };
    println!();

    group.size_new(parent);

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

    group.size_new(parent);

    assert_eq!(group.size.width, Unit::Auto(0));
    assert_eq!(group.size.height, Unit::Pixel(20));
    assert_eq!(group.size.widgets_left, Some(1));

    unsafe { RECURSE = 0 };
    println!();

    group.size_new(parent);

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

    group.size_new(parent);

    assert_eq!(group.size.width, Unit::Auto(20));
    assert_eq!(group.size.height, Unit::Pixel(20));
    assert_eq!(group.size.widgets_left, Some(2));

    // dbg!(&group.size);

    unsafe { RECURSE = 0 };
    println!();

    group.size_new(parent);

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

    group.size_new(parent);

    assert_eq!(group.size.width, Unit::Auto(20));
    assert_eq!(group.size.height, Unit::Pixel(20));
    assert_eq!(group.size.widgets_left, Some(1));

    unsafe { RECURSE = 0 };
    println!();

    group.size_new(parent);

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

#[test]
fn multi_subcontainer_left_right() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(LeftRight);
    group.size = size(0, 0, 800, 600);

    let mut subgroup = Group::new().direction(LeftRight);
    subgroup.children.push(Box::new(rect().w(20.percent()).h(20)));
    subgroup.children.push(Box::new(rect().w(80.percent()).h(20)));
    group.children.push(Box::new(subgroup));

    let mut subgroup2 = Group::new().direction(LeftRight);
    subgroup2.children.push(Box::new(rect().w(20.percent()).h(20)));
    subgroup2.children.push(Box::new(rect().w(80.percent()).h(20)));
    group.children.push(Box::new(subgroup2));

    let size = group.calculate_size(parent);

    // assert_eq!(size.width, Unit::Auto(20, 20));
    assert_eq!(size.height, Unit::Pixel(20));

    group.position_new(parent);

    //Parent container
    assert_eq!(group.size.width, Unit::Pixel(800));
    assert_eq!(group.size.height, Unit::Pixel(600));

    //First subcontainer
    let sg = group.children[0].size_mut().clone();
    assert_eq!(sg.width, Unit::Pixel(800));
    assert_eq!(sg.height, Unit::Pixel(600));

    let children = group.children[0].children();

    let h1r1 = children[0].size_mut().clone();
    assert_eq!(h1r1.x, Unit::Pixel(0));
    assert_eq!(h1r1.width, Unit::Pixel(160));
    assert_eq!(h1r1.height, Unit::Pixel(20));

    let h1r2 = children[1].size_mut().clone();
    assert_eq!(h1r2.x, Unit::Pixel(160));
    assert_eq!(h1r2.width, Unit::Pixel(640));
    assert_eq!(h1r2.height, Unit::Pixel(20));

    //Second subcontainer
    let children = group.children[1].children();

    let h1r1 = children[0].size_mut().clone();
    assert_eq!(h1r1.x, Unit::Pixel(800));
    assert_eq!(h1r1.width, Unit::Pixel(160));
    assert_eq!(h1r1.height, Unit::Pixel(20));

    let h1r2 = children[1].size_mut().clone();
    assert_eq!(h1r2.x, Unit::Pixel(960));
    assert_eq!(h1r2.width, Unit::Pixel(640));
    assert_eq!(h1r2.height, Unit::Pixel(20));
}

#[test]
fn mutli_plus_fixed_left_right() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(LeftRight);
    // group.size = size(0, 0, 800, 600);
    group.children.push(Box::new(rect().wh(20)));

    let mut subgroup = Group::new().direction(LeftRight);
    subgroup.children.push(Box::new(rect().w(50.percent()).h(20)));
    subgroup.children.push(Box::new(rect().w(50.percent()).h(20)));
    group.children.push(Box::new(subgroup));

    // let mut subgroup2 = Group::new().direction(LeftRight);
    // subgroup2.children.push(Box::new(rect().w(20.percent()).h(20)));
    // subgroup2.children.push(Box::new(rect().w(80.percent()).h(20)));
    // group.children.push(Box::new(subgroup2));

    let size = group.calculate_size(parent);

    assert_eq!(size.width, Unit::Auto(20));
    assert_eq!(size.height, Unit::Pixel(20));
    assert_eq!(size.widgets_left, Some(1));

    group.position_new(parent);

    //Parent container
    assert_eq!(group.size.width, Unit::Pixel(800));
    assert_eq!(group.size.height, Unit::Pixel(600));

    //Rectangle
    let rect = group.children[0].size_mut().clone();
    assert_eq!(rect.x, Unit::Pixel(0));
    assert_eq!(rect.y, Unit::Pixel(0));

    //First subcontainer
    let sg = group.children[1].size_mut().clone();
    assert_eq!(sg.x, Unit::Pixel(20));
    assert_eq!(sg.y, Unit::Pixel(0));
    assert_eq!(sg.width, Unit::Pixel(780));
    assert_eq!(sg.height, Unit::Pixel(20));

    let children = group.children[1].children();

    //First Rectangle
    let r1 = children[0].size_mut().clone();
    assert_eq!(r1.x, Unit::Pixel(20));
    assert_eq!(r1.y, Unit::Pixel(0));
    assert_eq!(r1.width, Unit::Pixel(390));
    assert_eq!(r1.height, Unit::Pixel(20));

    //Second Rectangle
    let r2 = children[1].size_mut().clone();
    assert_eq!(r2.x, Unit::Pixel(20 + 390));
    assert_eq!(r2.y, Unit::Pixel(0));
    assert_eq!(r2.width, Unit::Pixel(390));
    assert_eq!(r2.height, Unit::Pixel(20));
}
