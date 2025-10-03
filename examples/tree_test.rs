use softui::tree::*;

fn main() {
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
    tree.layout(id, original_parent_size, parent_pos);

    check_size(&tree, 0, 800.0, 600.0);
    check_size(&tree, 1, 800.0, 600.0);
    check_size(&tree, 2, 400.0 - 10.0, 300.0 - 10.0);
}
