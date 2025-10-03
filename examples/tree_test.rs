use softui::tree::*;

fn main() {
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

    tree.layout(0, [800.0, 600.0], [0.0, 0.0]);

    check_size(&tree, 0, 800.0, 600.0);
    check_size(&tree, 1, 800.0, 600.0);
    check_size(&tree, 2, 40.0, 40.0);
    check_size(&tree, 3, 800.0 - (2.0 * 10.0) - 40.0 - 10.0, 40.0);
}
