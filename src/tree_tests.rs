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

#[test]
fn min_max_with_fill_sibling() {
    let mut tree = Arena::new();

    let root = tree.alloc(Node::default());

    // Vertical parent
    let v_parent = tree.alloc(Node {
        direction: Direction::TopToBottom,
        ..Default::default()
    });
    tree.add_child(root, v_parent);

    // First horizontal row with min/max child and fill child
    let h_row1 = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(50.0)],
        direction: Direction::LeftToRight,
        ..Default::default()
    });
    tree.add_child(v_parent, h_row1);

    let min_max_child = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(50.0)],
        min_size: [Some(Unit::Fixed(100.0)), None],
        max_size: [Some(Unit::Fixed(200.0)), None],
        ..Default::default()
    });
    tree.add_child(h_row1, min_max_child);

    let fill_child = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(50.0)],
        ..Default::default()
    });
    tree.add_child(h_row1, fill_child);

    // Second horizontal row
    let h_row2 = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(50.0)],
        direction: Direction::LeftToRight,
        ..Default::default()
    });
    tree.add_child(v_parent, h_row2);

    let child2 = tree.alloc(Node {
        desired_size: [Unit::Fixed(200.0), Unit::Fixed(50.0)],
        ..Default::default()
    });
    tree.add_child(h_row2, child2);

    // Third horizontal row
    let h_row3 = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(50.0)],
        direction: Direction::LeftToRight,
        ..Default::default()
    });
    tree.add_child(v_parent, h_row3);

    let child3 = tree.alloc(Node {
        desired_size: [Unit::Fixed(100.0), Unit::Fixed(50.0)],
        ..Default::default()
    });
    tree.add_child(h_row3, child3);

    let nodes = unsafe { tree.as_mut_slice() };
    calculate_root_size(nodes, 0, [800.0, 600.0], [0.0, 0.0]);
    layout(nodes, 0);

    // First row children: min_max_child should respect min 100px, max 200px
    // With 800px available and 2 Fill children:
    // - Initial distribution: each gets 400px
    // - min_max_child hits max constraint of 200px
    // - Freed space (200px) is redistributed to fill_child

    // Expected behavior (after fix):
    check_size(nodes, 3, 200.0, 50.0);  // min_max_child clamped to max 200px
    check_size(nodes, 4, 600.0, 50.0);  // fill_child gets remaining space

    // crate::tree_simplier::draw_tree(nodes);
}

#[test]
fn min_max_constraints() {
    let mut tree = Arena::new();

    let root = tree.alloc(Node {
        direction: Direction::TopToBottom,
        ..Default::default()
    });

    // Horizontal container to test width constraints
    let h_parent = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(100.0)],
        direction: Direction::LeftToRight,
        gap: 10.0,
        ..Default::default()
    });
    tree.add_child(root, h_parent);

    let min_width_child = tree.alloc(Node {
        desired_size: [Unit::Fixed(50.0), Unit::Fixed(50.0)],
        min_size: [Some(Unit::Fixed(200.0)), None],
        ..Default::default()
    });
    tree.add_child(h_parent, min_width_child);

    let max_width_child = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(50.0)],
        max_size: [Some(Unit::Fixed(400.0)), None],
        ..Default::default()
    });
    tree.add_child(h_parent, max_width_child);

    // Vertical container to test height constraints
    let v_parent = tree.alloc(Node {
        desired_size: [Unit::Fixed(100.0), Unit::Fill],
        direction: Direction::TopToBottom,
        gap: 10.0,
        ..Default::default()
    });
    tree.add_child(root, v_parent);

    let min_height_child = tree.alloc(Node {
        desired_size: [Unit::Fixed(50.0), Unit::Fixed(50.0)],
        min_size: [None, Some(Unit::Fixed(150.0))],
        ..Default::default()
    });
    tree.add_child(v_parent, min_height_child);

    let max_height_child = tree.alloc(Node {
        desired_size: [Unit::Fixed(50.0), Unit::Fill],
        max_size: [None, Some(Unit::Fixed(100.0))],
        ..Default::default()
    });
    tree.add_child(v_parent, max_height_child);

    let nodes = unsafe { tree.as_mut_slice() };
    calculate_root_size(nodes, 0, [800.0, 600.0], [0.0, 0.0]);
    layout(nodes, 0);

    // Min width: desired 50px but expanded to min 200px
    check_size(nodes, 2, 200.0, 50.0);
    // Max width: Fill gets (800-200-10)=590px, clamped to max 400px
    check_size(nodes, 3, 400.0, 50.0);
    // Min height: desired 50px but expanded to min 150px
    check_size(nodes, 5, 50.0, 150.0);
    // Max height: Fill gets (500-150-10)=340px, clamped to max 100px
    check_size(nodes, 6, 50.0, 100.0);

    // crate::tree_simplier::draw_tree(nodes);
}

#[test]
fn cascading_fill_constraints() {
    let mut tree = Arena::new();

    let root = tree.alloc(Node::default());

    // Horizontal container with 4 Fill children, each with different constraints
    let container = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(100.0)],
        direction: Direction::LeftToRight,
        ..Default::default()
    });
    tree.add_child(root, container);

    // Child 1: Fill with max 100px
    let child1 = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(100.0)],
        max_size: [Some(Unit::Fixed(100.0)), None],
        ..Default::default()
    });
    tree.add_child(container, child1);

    // Child 2: Fill with max 150px (should hit constraint in second iteration)
    let child2 = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(100.0)],
        max_size: [Some(Unit::Fixed(150.0)), None],
        ..Default::default()
    });
    tree.add_child(container, child2);

    // Child 3: Fill with max 200px (should hit constraint in third iteration)
    let child3 = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(100.0)],
        max_size: [Some(Unit::Fixed(200.0)), None],
        ..Default::default()
    });
    tree.add_child(container, child3);

    // Child 4: Fill with no constraints (gets all remaining space)
    let child4 = tree.alloc(Node {
        desired_size: [Unit::Fill, Unit::Fixed(100.0)],
        ..Default::default()
    });
    tree.add_child(container, child4);

    let nodes = unsafe { tree.as_mut_slice() };
    calculate_root_size(nodes, 0, [800.0, 600.0], [0.0, 0.0]);
    layout(nodes, 0);

    // With 800px available and 4 Fill children:
    // Iteration 1: Each would get 200px
    //   - child1 hits max 100px, freed 100px, 3 children remain
    // Iteration 2: Each remaining gets (700/3) ≈ 233.33px
    //   - child2 hits max 150px, freed ~83.33px, 2 children remain
    // Iteration 3: Each remaining gets (550/2) = 275px
    //   - child3 hits max 200px, freed 75px, 1 child remains
    // Iteration 4: child4 gets all remaining = 350px

    check_size(nodes, 2, 100.0, 100.0);  // child1: clamped to max 100px
    check_size(nodes, 3, 150.0, 100.0);  // child2: clamped to max 150px
    check_size(nodes, 4, 200.0, 100.0);  // child3: clamped to max 200px
    check_size(nodes, 5, 350.0, 100.0);  // child4: gets remaining space

    // crate::tree_simplier::draw_tree(nodes);
}