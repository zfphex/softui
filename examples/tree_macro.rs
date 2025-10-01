#![allow(unused)]
use softui::{flext, groupt, tree::*, tree_simplier::*};

fn check_size(tree: &Tree, id: usize, w: f32, h: f32) {
    let node = &tree.nodes[id];
    assert_eq!(node.size[0], w, "width {} != {}", node.size[0], w);
    assert_eq!(node.size[1], h, "height {} != {}", node.size[1], h);
}

fn main() {
    let mut tree = flext!(groupt!(
        rect().w(100).h(40),
        rect().wfill().h(40),
        rect().w(80).h(40),
        rect().w(80).h(40),
    ));

    //Node 0 is always the root/first node.
    //Force update the root node to inherit the window size.
    tree.nodes[0].size = [800.0, 60.0];
    tree.layout(0, [800.0, 60.0], [0.0, 0.0]);

    //Window node
    check_size(&tree, 0, 800.0, 60.0);

    //Container node
    check_size(&tree, 1, 800.0, 60.0);
    check_size(&tree, 2, 100.0, 40.0);
    check_size(&tree, 3, 540.0, 40.0);
    check_size(&tree, 4, 80.0, 40.0);
    check_size(&tree, 5, 80.0, 40.0);

    //
    //
    //

    let mut tree = Tree::new();

    let nav = tree.add_node(Unit::Fixed(800.0), Unit::Fixed(60.0), Direction::LeftToRight, 0.0);
    let logo = tree.add_node(Unit::Fixed(100.0), Unit::Fixed(40.0), Direction::LeftToRight, 0.0);
    let spacer = tree.add_node(Unit::Fill, Unit::Fixed(40.0), Direction::LeftToRight, 0.0);
    let btn1 = tree.add_node(Unit::Fixed(80.0), Unit::Fixed(40.0), Direction::LeftToRight, 0.0);
    let btn2 = tree.add_node(Unit::Fixed(80.0), Unit::Fixed(40.0), Direction::LeftToRight, 0.0);

    tree.add_child(nav, logo);
    tree.add_child(nav, spacer);
    tree.add_child(nav, btn1);
    tree.add_child(nav, btn2);

    tree.layout(nav, [800.0, 60.0], [0.0, 0.0]);

    // Spacer gets: 800 - 100 - 80 - 80 = 540

    check_size(&tree, nav, 800.0, 60.0);
    check_size(&tree, logo, 100.0, 40.0);
    check_size(&tree, spacer, 540.0, 40.0);
    check_size(&tree, btn1, 80.0, 40.0);
    check_size(&tree, btn2, 80.0, 40.0);
}
