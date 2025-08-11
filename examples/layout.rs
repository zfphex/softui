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

pub enum Size {
    Fixed(usize, usize),
    FillEvenly(usize, usize),
}

fn main() {
    // let window = Rect::new(0, 0, 800, 600);
    let margin = 0;
    let total_area = Rect::new(margin, margin, 800, 600);

    let mut group = Group::new();
    group.direction = TopBottom;
    group.area_new = urect(0, 0, 800, 600);

    //(320, 200)
    //(160, 200)
    //(320, 200)
    //(800, 0, 200, 200)
    let mut subgroup = Group::new();
    subgroup.direction = TopBottom;

    // subgroup.children.push(Box::new(rect().w_new(40.percent()).h_new(200)));
    // subgroup.children.push(Box::new(rect().w_new(20.percent()).h_new(200)));
    // subgroup.children.push(Box::new(rect().w_new(40.percent()).h_new(200)));

    group.children.push(Box::new(subgroup));

    // group.children.push(Box::new(rect().wh_new(200)));

    // Works just fine
    // group.children.push(Box::new(rect().w_new(40.percent()).h_new(200)));
    // group.children.push(Box::new(rect().w_new(20.percent()).h_new(200)));
    // group.children.push(Box::new(rect().w_new(40.percent()).h_new(200)));

    // Works just fine.


    dbg!(&group.children);
    let size = group.size_new(total_area);
    group.layout_new(size, total_area);

    let mut commands = Vec::new();
    group.draw(&mut commands, None);

    dbg!(commands);
}
