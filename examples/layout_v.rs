use softui::*;

//(800, 600)
//Flex(Rect(Fill), Rect(Fill))
//If two widgets want to fill, they should be spaced evenly inside the parent area:
//Given that flex is top to bottom.
//Rect(0, 0, 400, 300)
//Rect(0, 300, 400, 300)

//(800, 600)
//Flex(Rect(Fill), Rect(200, 200), Rect(Fill))
//This requires that the both fill rectangles have a size of (0, 0).
//In the first pass the size is calculated as (200, 200).
//In the second pass the items tagged as fill are calculated based on the remaining area.
//Remaining Width = (Width - Used Width) = 600
//Fill Width = Remaining Width / Remaining Widgets = 600 / 2 = 300

//Rect(0, 0, 300, 200)
//Rect(0, 0, 200, 200)
//Rect(0, 0, 300, 200)

//Find the size first
//Then set the position

//The root container always has a fixed size but a child container could ask to fill the avaliable space.

fn main() {
    let parent = Rect::new(0, 0, 800, 600);

    let mut group = Group::new().direction(TopBottom);
    group.size = size(0, 0, 800, 600);

    // let mut subgroup2 = Group::new().direction(LeftRight);
    // subgroup2.children.push(Box::new(rect().w(20.percent()).h(20)));
    // subgroup2.children.push(Box::new(rect().w(80.percent()).h(20)));
    // group.children.push(Box::new(subgroup2));

    let mut subgroup = Group::new().direction(LeftRight);
    subgroup.children.push(Box::new(rect().w(20.percent()).h(20)));
    // subgroup.children.push(Box::new(rect().w(80.percent()).h(20)));

    group.children.push(Box::new(subgroup));

    // group.children.push(Box::new(rect().wh(20)));

    let size = group.calculate_size(parent);

    group.position(size, parent);
    // dbg!(group.children);

    assert_eq!(group.size.height, Unit::Pixel(20));
}
