use crate::tree::*;

#[test]
fn fill() {
    let mut tree = Tree::new();

    //Window root container
    let root = tree.add_node(Node::default());

    //Child containers
    let parent = tree.add_node(
        //
        Node {
            gap: 10.0,
            padding: Amount::splat(10.0),
            ..Default::default()
        },
    );
    tree.add_child(root, parent);

    let fixed = tree.add_node(Node {
        desired_size: [Unit::Fixed(40.0), Unit::Fixed(40.0)],
        ..Default::default()
    });
    let fill = tree.add_node(Node {
        desired_size: [Unit::Fill, Unit::Fixed(40.0)],
        ..Default::default()
    });
    tree.add_child(parent, fixed);
    tree.add_child(parent, fill);

    tree.calculate_root_size(0, [800.0, 600.0], [0.0, 0.0]);
    tree.layout(0);

    check_size(&tree, 0, 800.0, 600.0);
    check_size(&tree, 1, 800.0, 600.0);
    check_size(&tree, 2, 40.0, 40.0);
    check_size(&tree, 3, 800.0 - (2.0 * 10.0) - 40.0 - 10.0, 40.0);

    // crate::tree_simplier::draw_tree(tree);
}

#[test]
fn percentage() {
    let mut tree = Tree::new();

    //Window root container
    let root = tree.add_node(Node::default());

    //Child containers

    let parent = tree.add_node(Node {
        desired_size: [Unit::Fill, Unit::Fill],
        gap: 10.0,
        padding: Amount::splat(10.0),
        ..Default::default()
    });
    tree.add_child(root, parent);

    let percent = tree.add_node(Node {
        desired_size: [Unit::Percentage(50.0), Unit::Percentage(50.0)],
        ..Default::default()
    });

    tree.add_child(parent, percent);

    let id = 0;
    let original_parent_size = [800.0, 600.0];
    let parent_pos = [0.0, 0.0];

    tree.calculate_root_size(id, original_parent_size, parent_pos);
    tree.layout(id);

    check_size(&tree, 0, 800.0, 600.0);
    check_size(&tree, 1, 800.0, 600.0);
    check_size(&tree, 2, 400.0 - 10.0, 300.0 - 10.0);

    // crate::tree_simplier::draw_tree(tree);
}
