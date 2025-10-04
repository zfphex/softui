use softui::tree::*;

fn main() {
    let mut tree = Tree::new();

    //Window root container
    let root = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0, Amount::splat(0.0));

    //Child containers
    let parent = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 2.0, Amount::splat(10.0));
    tree.add_child(root, parent);

    for _ in 0..100 {
        let child = tree.add_node(Unit::Fill, Unit::Percentage(40.0), Direction::LeftToRight, 0.0, Amount::splat(0.0));
        tree.add_child(parent, child);
    }

    let id = 0;
    let original_parent_size = [800.0, 600.0];
    let parent_pos = [0.0, 0.0];

    tree.calculate_root_size(id, original_parent_size, parent_pos);
    tree.layout(id);

    check_size(&tree, 0, 800.0, 600.0);
    check_size(&tree, 1, 800.0, 600.0);
    dbg!(&tree.nodes[2]);
}
