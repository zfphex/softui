use crate::tree::*;

#[test]
fn fill() {
    let mut tree = Tree::new();

    //Window root container
    let root = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0, 0.0);

    //Child containers
    let parent = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 10.0, 10.0);
    tree.add_child(root, parent);

    let fixed = tree.add_node(Unit::Fixed(40.0), Unit::Fixed(40.0), Direction::LeftToRight, 0.0, 0.0);
    let fill = tree.add_node(Unit::Fill, Unit::Fixed(40.0), Direction::LeftToRight, 0.0, 0.0);
    tree.add_child(parent, fixed);
    tree.add_child(parent, fill);

    tree.calculate_root_size(0, [800.0, 600.0], [0.0, 0.0]);
    tree.layout(0);

    check_size(&tree, 0, 800.0, 600.0);
    check_size(&tree, 1, 800.0, 600.0);
    check_size(&tree, 2, 40.0, 40.0);
    check_size(&tree, 3, 800.0 - (2.0 * 10.0) - 40.0 - 10.0, 40.0);
}

#[test]
fn percentage() {
    let mut tree = Tree::new();

    //Window root container
    let root = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0, 0.0);

    //Child containers
    let parent = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 10.0, 10.0);
    tree.add_child(root, parent);

    let percent = tree.add_node(
        Unit::Percentage(50.0),
        Unit::Percentage(50.0),
        Direction::LeftToRight,
        0.0,
        0.0,
    );

    tree.add_child(parent, percent);

    let id = 0;
    let original_parent_size = [800.0, 600.0];
    let parent_pos = [0.0, 0.0];

    tree.calculate_root_size(id, original_parent_size, parent_pos);
    tree.layout(id);

    check_size(&tree, 0, 800.0, 600.0);
    check_size(&tree, 1, 800.0, 600.0);
    check_size(&tree, 2, 400.0 - 10.0, 300.0 - 10.0);
}
