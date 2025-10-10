#![allow(unused)]
use softui::{
    tree::{calculate_root_size, check_size, layout, Amount, Node, Unit},
    Arena,
};

fn main() {
    let tree = Arena::new();

    //Window root container
    let root = tree.alloc(Node {
        desired_size: [Unit::Fixed(800.0), Unit::Fixed(600.0)],
        ..Default::default()
    });

    let container = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fill],
        ..Default::default()
    });

    tree.add_child(root, container);

    let max = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(50.0)],
        min_size: [Some(Unit::Fixed(50.0)), None],
        ..Default::default()
    });

    tree.add_child(container, max);

    let nodes = unsafe { tree.as_mut_slice() };
    calculate_root_size(nodes, 0, [800.0, 600.0], [0.0, 0.0]);
    layout(nodes, 0);

    check_size(nodes, 0, 800.0, 600.0);
    check_size(nodes, 1, 800.0, 600.0);
    check_size(nodes, 2, 800.0, 50.0);

    // crate::tree_simplier::draw_tree(nodes);
}
