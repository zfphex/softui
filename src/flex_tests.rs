use crate::*;

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
    subgroup.children.push(Box::new(rect().w(20.percent()).h(20)));
    // subgroup.children.push(Box::new(rect().w(80.percent()).h(20)));
    group.children.push(Box::new(subgroup));

    // let mut subgroup2 = Group::new().direction(LeftRight);
    // subgroup2.children.push(Box::new(rect().w(20.percent()).h(20)));
    // subgroup2.children.push(Box::new(rect().w(80.percent()).h(20)));
    // group.children.push(Box::new(subgroup2));

    let size = group.calculate_size(parent);

    for child in &mut group.children {
        dbg!(child.calculate_size(parent));
    }

    return;

    // dbg!(&size);

    assert_eq!(size.width, Unit::Auto(20));
    assert_eq!(size.height, Unit::Auto(20));
    assert_eq!(size.remaining_widgets, Some(1));

    group.position_new(parent);

    //Parent container
    assert_eq!(group.size.width, Unit::Pixel(800));
    assert_eq!(group.size.height, Unit::Pixel(600));

    //First subcontainer
    let sg = group.children[0].size_mut().clone();
    assert_eq!(sg.width, Unit::Pixel(780));
    assert_eq!(sg.height, Unit::Pixel(20));
}
