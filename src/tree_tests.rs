use crate::{tree::*, Arena};

#[test]
fn fill() {
    let mut tree = Arena::new();

    //Window root container
    let root = tree.alloc(Node::default());

    //Child containers
    let parent = tree.alloc(Node {
        gap: 10.0,
        padding: Amount::splat(10.0),
        ..Default::default()
    });
    tree.add_child(root, parent);

    let fixed = tree.alloc(Node {
        desired_size: [Unit::Fixed(40.0), Unit::Fixed(40.0)],
        ..Default::default()
    });
    let fill = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(40.0)],
        ..Default::default()
    });
    tree.add_child(parent, fixed);
    tree.add_child(parent, fill);

    let id = 0;
    let original_parent_size = [800.0, 600.0];
    let parent_pos = [0.0, 0.0];

    let nodes = unsafe { tree.as_mut_slice() };
    calculate_root_size(nodes, id, original_parent_size, parent_pos);
    layout(nodes, id);

    check_size(nodes, 0, 800.0, 600.0);
    check_size(nodes, 1, 800.0, 600.0);
    check_size(nodes, 2, 40.0, 40.0);
    check_size(nodes, 3, 800.0 - (2.0 * 10.0) - 40.0 - 10.0, 40.0);

    // crate::tree_simplier::draw_tree(nodes);
}

#[test]
fn percentage() {
    let mut tree = Arena::new();

    //Window root container
    let root = tree.alloc(Node::default());

    //Child containers

    let parent = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fill],
        gap: 10.0,
        padding: Amount::splat(10.0),
        ..Default::default()
    });
    tree.add_child(root, parent);

    let percent = tree.alloc(Node {
        desired_size: [Unit::Percentage(50.0), Unit::Percentage(50.0)],
        ..Default::default()
    });

    tree.add_child(parent, percent);

    let id = 0;
    let original_parent_size = [800.0, 600.0];
    let parent_pos = [0.0, 0.0];

    let nodes = unsafe { tree.as_mut_slice() };
    calculate_root_size(nodes, id, original_parent_size, parent_pos);
    layout(nodes, id);

    check_size(nodes, 0, 800.0, 600.0);
    check_size(nodes, 1, 800.0, 600.0);
    check_size(nodes, 2, 400.0 - 10.0, 300.0 - 10.0);

    // crate::tree_simplier::draw_tree(nodes);
}
