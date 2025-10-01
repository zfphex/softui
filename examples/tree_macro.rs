#![allow(unused)]
use softui::{flext, groupt, ht, tree::*, vt};

fn check_size(tree: &Tree, id: usize, w: f32, h: f32) {
    let node = &tree.nodes[id];
    assert_eq!(node.size[0], w, "width {} != {}", node.size[0], w);
    assert_eq!(node.size[1], h, "height {} != {}", node.size[1], h);
}

fn main() {
    // let nav = Node::new(Sizing::Fixed(800.0), Sizing::Fixed(60.0), Direction::LeftToRight, 0.0);
    let mut tree = flext!(groupt!(
        Node::new(Sizing::Fixed(100.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0),
        Node::new(Sizing::Fill, Sizing::Fixed(40.0), Direction::LeftToRight, 0.0),
        Node::new(Sizing::Fixed(80.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0),
        Node::new(Sizing::Fixed(80.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0),
    ));

    //Node 0 is always the root/first node.
    //Force update the root node to inherit the window size.
    tree.nodes[0].size = [800.0, 60.0];
    tree.layout(0, [800.0, 60.0], [0.0, 0.0]);

    check_size(&tree, 1, 100.0, 40.0);
    check_size(&tree, 2, 540.0, 40.0);
    check_size(&tree, 3, 80.0, 40.0);
    check_size(&tree, 4, 80.0, 40.0);

    return;

    //
    //
    //

    let mut tree = Tree::new();

    // let nav = tree.add_node(Sizing::Fixed(800.0), Sizing::Fixed(60.0), Direction::LeftToRight, 0.0);
    // let logo = tree.add_node(Sizing::Fixed(100.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);
    // let spacer = tree.add_node(Sizing::Fill, Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);
    // let btn1 = tree.add_node(Sizing::Fixed(80.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);
    // let btn2 = tree.add_node(Sizing::Fixed(80.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);

    // tree.add_child(nav, logo);
    // tree.add_child(nav, spacer);
    // tree.add_child(nav, btn1);
    // tree.add_child(nav, btn2);

    let nav = Node::new(Sizing::Fixed(800.0), Sizing::Fixed(60.0), Direction::LeftToRight, 0.0);
    let logo = Node::new(Sizing::Fixed(100.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);
    let spacer = Node::new(Sizing::Fill, Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);
    let btn1 = Node::new(Sizing::Fixed(80.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);
    let btn2 = Node::new(Sizing::Fixed(80.0), Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);

    let nodes = groupt!(logo, spacer, btn1, btn2);

    let nav = tree.add_node_new(nav);
    tree.add_children(nav, nodes);

    tree.layout(nav, [800.0, 60.0], [0.0, 0.0]);

    // Spacer gets: 800 - 100 - 80 - 80 = 540

    // check_size(&tree, logo, 100.0, 40.0);
    // check_size(&tree, spacer, 540.0, 40.0);
    // check_size(&tree, btn1, 80.0, 40.0);
    // check_size(&tree, btn2, 80.0, 40.0);

    check_size(&tree, 1, 100.0, 40.0);
    check_size(&tree, 2, 540.0, 40.0);
    check_size(&tree, 3, 80.0, 40.0);
    check_size(&tree, 4, 80.0, 40.0);
}
