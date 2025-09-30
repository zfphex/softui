use crate::tree::*;

fn check_size(tree: &Tree, id: usize, w: f32, h: f32) {
    let node = &tree.nodes[id];
    assert_eq!(node.size[0], w, "{}: width {} != {}", node.name, node.size[0], w);
    assert_eq!(node.size[1], h, "{}: height {} != {}", node.name, node.size[1], h);
}

fn check_pos(tree: &Tree, id: usize, x: f32, y: f32) {
    let node = &tree.nodes[id];
    assert_eq!(node.pos[0], x, "{}: x {} != {}", node.name, node.pos[0], x);
    assert_eq!(node.pos[1], y, "{}: y {} != {}", node.name, node.pos[1], y);
}

#[test]
fn test_left_to_right() {
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let b1 = tree.add_node(
        "B1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );
    let b2 = tree.add_node(
        "B2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(40.0),
        Direction::LeftToRight,
        0.0,
    );
    let b3 = tree.add_node("B3", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, b1);
    tree.add_child(p, b2);
    tree.add_child(p, b3);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    check_size(&tree, b1, 30.0, 30.0);
    check_pos(&tree, b1, 0.0, 0.0);
    check_size(&tree, b2, 40.0, 40.0);
    check_pos(&tree, b2, 30.0, 0.0);
    check_size(&tree, b3, 30.0, 50.0);
    check_pos(&tree, b3, 70.0, 0.0);
}

#[test]
fn test_right_to_left() {
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::RightToLeft,
        0.0,
    );
    let b1 = tree.add_node(
        "B1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::RightToLeft,
        0.0,
    );
    let b2 = tree.add_node(
        "B2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(40.0),
        Direction::RightToLeft,
        0.0,
    );
    let b3 = tree.add_node("B3", Sizing::Fill, Sizing::Fixed(50.0), Direction::RightToLeft, 0.0);

    tree.add_child(p, b1);
    tree.add_child(p, b2);
    tree.add_child(p, b3);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    check_size(&tree, b1, 30.0, 30.0);
    check_pos(&tree, b1, 70.0, 0.0);
    check_size(&tree, b2, 40.0, 40.0);
    check_pos(&tree, b2, 30.0, 0.0);
    check_size(&tree, b3, 30.0, 50.0);
    check_pos(&tree, b3, 0.0, 0.0);
}

#[test]
fn test_fill_with_nested() {
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let r1 = tree.add_node(
        "R1",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let h1 = tree.add_node("H1", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let r2 = tree.add_node(
        "R2",
        Sizing::Percentage(50.0),
        Sizing::Percentage(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let h2 = tree.add_node("H2", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);

    tree.add_child(p, r1);
    tree.add_child(p, h1);
    tree.add_child(p, h2);
    tree.add_child(h1, r2);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    check_size(&tree, p, 100.0, 100.0);
    check_pos(&tree, p, 0.0, 0.0);
    check_size(&tree, r1, 50.0, 50.0);
    check_pos(&tree, r1, 0.0, 0.0);
    check_size(&tree, h1, 25.0, 100.0);
    check_pos(&tree, h1, 50.0, 0.0);
    check_size(&tree, r2, 12.5, 50.0);
    check_pos(&tree, r2, 50.0, 0.0);
    check_size(&tree, h2, 25.0, 100.0);
    check_pos(&tree, h2, 75.0, 0.0);
}

#[test]
fn test_top_to_bottom() {
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::TopToBottom,
        0.0,
    );
    let b1 = tree.add_node(
        "B1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::TopToBottom,
        0.0,
    );
    let b2 = tree.add_node(
        "B2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(40.0),
        Direction::TopToBottom,
        0.0,
    );
    let b3 = tree.add_node("B3", Sizing::Fixed(50.0), Sizing::Fill, Direction::TopToBottom, 0.0);

    tree.add_child(p, b1);
    tree.add_child(p, b2);
    tree.add_child(p, b3);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    check_size(&tree, b1, 30.0, 30.0);
    check_pos(&tree, b1, 0.0, 0.0);
    check_size(&tree, b2, 40.0, 40.0);
    check_pos(&tree, b2, 0.0, 30.0);
    check_size(&tree, b3, 50.0, 30.0);
    check_pos(&tree, b3, 0.0, 70.0);
}

#[test]
fn test_bottom_to_top() {
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::BottomToTop,
        0.0,
    );
    let b1 = tree.add_node(
        "B1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::BottomToTop,
        0.0,
    );
    let b2 = tree.add_node(
        "B2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(40.0),
        Direction::BottomToTop,
        0.0,
    );
    let b3 = tree.add_node("B3", Sizing::Fixed(50.0), Sizing::Fill, Direction::BottomToTop, 0.0);

    tree.add_child(p, b1);
    tree.add_child(p, b2);
    tree.add_child(p, b3);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    check_size(&tree, b1, 30.0, 30.0);
    check_pos(&tree, b1, 0.0, 70.0);
    check_size(&tree, b2, 40.0, 40.0);
    check_pos(&tree, b2, 0.0, 30.0);
    check_size(&tree, b3, 50.0, 30.0);
    check_pos(&tree, b3, 0.0, 0.0);
}

#[test]
fn test_nested_overflow() {
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let h1 = tree.add_node(
        "H1",
        Sizing::Percentage(50.0),
        Sizing::Percentage(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let r = tree.add_node(
        "R",
        Sizing::Percentage(50.0),
        Sizing::Percentage(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let h2 = tree.add_node(
        "H2",
        Sizing::Percentage(100.0),
        Sizing::Percentage(100.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, h1);
    tree.add_child(p, h2);
    tree.add_child(h1, r);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    check_size(&tree, p, 100.0, 100.0);
    check_size(&tree, h1, 50.0, 50.0);
    check_size(&tree, r, 25.0, 25.0);
    check_size(&tree, h2, 100.0, 100.0);
}

// Removed test_nested_percentage - covered by test_deeply_nested_percentages

#[test]
fn test_fit_with_fill() {
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let g1 = tree.add_node("G1", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let g2 = tree.add_node("G2", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let r = tree.add_node(
        "R",
        Sizing::Fixed(10.0),
        Sizing::Fixed(10.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, g1);
    tree.add_child(g1, g2);
    tree.add_child(g2, r);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    check_size(&tree, p, 100.0, 100.0);
    check_pos(&tree, p, 0.0, 0.0);
    check_size(&tree, g1, 100.0, 100.0);
    check_pos(&tree, g1, 0.0, 0.0);
    check_size(&tree, g2, 10.0, 10.0);
    check_pos(&tree, g2, 0.0, 0.0);
    check_size(&tree, r, 10.0, 10.0);
    check_pos(&tree, r, 0.0, 0.0);
}

#[test]
#[should_panic(expected = "Fit containers cannot have Percentage or Fill children")]
fn test_invalid_fit_with_percentage() {
    let mut tree = Tree::new();
    let h1 = tree.add_node("H1", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let r = tree.add_node(
        "R",
        Sizing::Percentage(50.0),
        Sizing::Fixed(10.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(h1, r);

    tree.layout(h1, [100.0, 100.0], [0.0, 0.0]);
}

#[test]
#[should_panic(expected = "Fit containers cannot have Percentage or Fill children")]
fn test_invalid_fit_with_fill() {
    let mut tree = Tree::new();
    let h1 = tree.add_node("H1", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let r = tree.add_node("R", Sizing::Fill, Sizing::Fixed(10.0), Direction::LeftToRight, 0.0);

    tree.add_child(h1, r);

    tree.layout(h1, [100.0, 100.0], [0.0, 0.0]);
}

#[test]
fn test_multiple_fill_equal_distribution() {
    // Multiple Fill children should divide remaining space equally
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let f1 = tree.add_node("F1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let f2 = tree.add_node("F2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let f3 = tree.add_node("F3", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, f1);
    tree.add_child(p, f2);
    tree.add_child(p, f3);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // 200 / 3 = 66.666... for each
    check_size(&tree, f1, 200.0 / 3.0, 50.0);
    check_size(&tree, f2, 200.0 / 3.0, 50.0);
    check_size(&tree, f3, 200.0 / 3.0, 50.0);
    check_pos(&tree, f1, 0.0, 0.0);
    check_pos(&tree, f2, 200.0 / 3.0, 0.0);
    check_pos(&tree, f3, 400.0 / 3.0, 0.0);
}

#[test]
fn test_fill_with_fixed_leftovers() {
    // Fill gets only leftover space after Fixed allocations
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let r1 = tree.add_node(
        "R1",
        Sizing::Fixed(60.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let r2 = tree.add_node(
        "R2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let f1 = tree.add_node("F1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, r1);
    tree.add_child(p, r2);
    tree.add_child(p, f1);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // Fill gets 200 - 60 - 40 = 100
    check_size(&tree, f1, 100.0, 50.0);
    check_pos(&tree, f1, 100.0, 0.0);
}

#[test]
fn test_fill_with_percentage_leftovers() {
    // Fill gets leftover after Percentage allocations
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let pct = tree.add_node(
        "Pct",
        Sizing::Percentage(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let f1 = tree.add_node("F1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let f2 = tree.add_node("F2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, pct);
    tree.add_child(p, f1);
    tree.add_child(p, f2);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // Percentage gets 100, Fill children split remaining 100
    check_size(&tree, pct, 100.0, 50.0);
    check_size(&tree, f1, 50.0, 50.0);
    check_size(&tree, f2, 50.0, 50.0);
    check_pos(&tree, pct, 0.0, 0.0);
    check_pos(&tree, f1, 100.0, 0.0);
    check_pos(&tree, f2, 150.0, 0.0);
}

#[test]
fn test_oversubscribed_percentage() {
    // Percentages > 100% cause overflow (like CSS flex-basis)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let p1 = tree.add_node(
        "P1",
        Sizing::Percentage(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let p2 = tree.add_node(
        "P2",
        Sizing::Percentage(75.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, p1);
    tree.add_child(p, p2);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    // Total = 125 (50% + 75% = 125% of 100), exceeds parent
    check_size(&tree, p1, 50.0, 50.0);
    check_size(&tree, p2, 75.0, 50.0);
    check_pos(&tree, p1, 0.0, 0.0);
    check_pos(&tree, p2, 50.0, 0.0);
}

#[test]
fn test_fill_shrinks_to_zero_on_overflow() {
    // When Fixed + Percentage oversubscribe, Fill shrinks to 0
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let r1 = tree.add_node(
        "R1",
        Sizing::Fixed(60.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let p1 = tree.add_node(
        "P1",
        Sizing::Percentage(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let f1 = tree.add_node("F1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, r1);
    tree.add_child(p, p1);
    tree.add_child(p, f1);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    // 60 + 50 = 110 > 100, Fill gets 0
    check_size(&tree, f1, 0.0, 50.0);
}

#[test]
fn test_holy_grail_layout() {
    // Classic CSS holy grail: header, 3-column content, footer
    let mut tree = Tree::new();
    let root = tree.add_node(
        "Root",
        Sizing::Fixed(600.0),
        Sizing::Fixed(400.0),
        Direction::TopToBottom,
        0.0,
    );
    let header = tree.add_node("Header", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let content = tree.add_node("Content", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let footer = tree.add_node("Footer", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    let sidebar_left = tree.add_node(
        "SidebarL",
        Sizing::Fixed(100.0),
        Sizing::Fill,
        Direction::LeftToRight,
        0.0,
    );
    let main = tree.add_node("Main", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let sidebar_right = tree.add_node(
        "SidebarR",
        Sizing::Fixed(100.0),
        Sizing::Fill,
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(root, header);
    tree.add_child(root, content);
    tree.add_child(root, footer);
    tree.add_child(content, sidebar_left);
    tree.add_child(content, main);
    tree.add_child(content, sidebar_right);

    tree.layout(root, [600.0, 400.0], [0.0, 0.0]);

    check_size(&tree, header, 600.0, 50.0);
    check_size(&tree, footer, 600.0, 50.0);
    check_size(&tree, content, 600.0, 300.0);
    check_size(&tree, sidebar_left, 100.0, 300.0);
    check_size(&tree, main, 400.0, 300.0);
    check_size(&tree, sidebar_right, 100.0, 300.0);
    check_pos(&tree, header, 0.0, 0.0);
    check_pos(&tree, content, 0.0, 50.0);
    check_pos(&tree, footer, 0.0, 350.0);
}

// Removed test_flex_like_grow_pattern - redundant with test_multiple_fill_equal_distribution

// Removed test_cross_axis_fill - superseded by test_cross_axis_fill_all_directions

#[test]
fn test_nested_direction_change() {
    // Parent is horizontal, child is vertical
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::LeftToRight,
        0.0,
    );
    let v_container = tree.add_node(
        "V",
        Sizing::Fixed(100.0),
        Sizing::Fixed(200.0),
        Direction::TopToBottom,
        0.0,
    );
    let v1 = tree.add_node(
        "V1",
        Sizing::Fixed(80.0),
        Sizing::Fixed(60.0),
        Direction::TopToBottom,
        0.0,
    );
    let v2 = tree.add_node("V2", Sizing::Fixed(80.0), Sizing::Fill, Direction::TopToBottom, 0.0);

    tree.add_child(p, v_container);
    tree.add_child(v_container, v1);
    tree.add_child(v_container, v2);

    tree.layout(p, [200.0, 200.0], [0.0, 0.0]);

    check_size(&tree, v_container, 100.0, 200.0);
    check_size(&tree, v1, 80.0, 60.0);
    check_size(&tree, v2, 80.0, 140.0);
    check_pos(&tree, v1, 0.0, 0.0);
    check_pos(&tree, v2, 0.0, 60.0);
}

#[test]
fn test_deeply_nested_percentages() {
    // Test percentage calculations through multiple levels (also covers test_nested_percentage)
    let mut tree = Tree::new();
    let l1 = tree.add_node(
        "L1",
        Sizing::Fixed(1000.0),
        Sizing::Fixed(1000.0),
        Direction::LeftToRight,
        0.0,
    );
    let l2 = tree.add_node(
        "L2",
        Sizing::Percentage(80.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let l3 = tree.add_node(
        "L3",
        Sizing::Percentage(50.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let l4 = tree.add_node(
        "L4",
        Sizing::Percentage(25.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(l1, l2);
    tree.add_child(l2, l3);
    tree.add_child(l3, l4);

    tree.layout(l1, [1000.0, 1000.0], [0.0, 0.0]);

    check_size(&tree, l2, 800.0, 100.0);
    check_pos(&tree, l2, 0.0, 0.0);
    check_size(&tree, l3, 400.0, 100.0);
    check_pos(&tree, l3, 0.0, 0.0);
    check_size(&tree, l4, 100.0, 100.0);
    check_pos(&tree, l4, 0.0, 0.0);
}

#[test]
fn test_fit_nested_horizontal_sum() {
    // Fit container with horizontal children should sum their widths
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fixed(100.0), Direction::LeftToRight, 0.0);
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c3 = tree.add_node(
        "C3",
        Sizing::Fixed(25.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, c1);
    tree.add_child(fit, c2);
    tree.add_child(fit, c3);

    tree.layout(p, [200.0, 200.0], [0.0, 0.0]);

    // Fit should be 30 + 40 + 25 = 95
    check_size(&tree, fit, 95.0, 100.0);
}

#[test]
fn test_fit_nested_vertical_max() {
    // Fit container with vertical children should use max height
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::TopToBottom,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fixed(100.0), Sizing::Fit, Direction::TopToBottom, 0.0);
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(50.0),
        Sizing::Fixed(30.0),
        Direction::TopToBottom,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(50.0),
        Sizing::Fixed(40.0),
        Direction::TopToBottom,
        0.0,
    );
    let c3 = tree.add_node(
        "C3",
        Sizing::Fixed(50.0),
        Sizing::Fixed(25.0),
        Direction::TopToBottom,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, c1);
    tree.add_child(fit, c2);
    tree.add_child(fit, c3);

    tree.layout(p, [200.0, 200.0], [0.0, 0.0]);

    // Fit should sum heights: 30 + 40 + 25 = 95
    check_size(&tree, fit, 100.0, 95.0);
}

#[test]
fn test_fit_cross_axis_max() {
    // Fit in cross axis should take max of children
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(80.0),
        Direction::LeftToRight,
        0.0,
    );
    let c3 = tree.add_node(
        "C3",
        Sizing::Fixed(25.0),
        Sizing::Fixed(60.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, c1);
    tree.add_child(fit, c2);
    tree.add_child(fit, c3);

    tree.layout(p, [200.0, 200.0], [0.0, 0.0]);

    // Width = 30 + 40 + 25 = 95, Height = max(50, 80, 60) = 80
    check_size(&tree, fit, 95.0, 80.0);
}

#[test]
fn test_navbar_with_spacer() {
    // Common pattern: left items, spacer (Fill), right items
    let mut tree = Tree::new();
    let nav = tree.add_node(
        "Nav",
        Sizing::Fixed(800.0),
        Sizing::Fixed(60.0),
        Direction::LeftToRight,
        0.0,
    );
    let logo = tree.add_node(
        "Logo",
        Sizing::Fixed(100.0),
        Sizing::Fixed(40.0),
        Direction::LeftToRight,
        0.0,
    );
    let spacer = tree.add_node("Spacer", Sizing::Fill, Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);
    let btn1 = tree.add_node(
        "Btn1",
        Sizing::Fixed(80.0),
        Sizing::Fixed(40.0),
        Direction::LeftToRight,
        0.0,
    );
    let btn2 = tree.add_node(
        "Btn2",
        Sizing::Fixed(80.0),
        Sizing::Fixed(40.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(nav, logo);
    tree.add_child(nav, spacer);
    tree.add_child(nav, btn1);
    tree.add_child(nav, btn2);

    tree.layout(nav, [800.0, 60.0], [0.0, 0.0]);

    // Spacer gets: 800 - 100 - 80 - 80 = 540
    check_size(&tree, logo, 100.0, 40.0);
    check_size(&tree, spacer, 540.0, 40.0);
    check_size(&tree, btn1, 80.0, 40.0);
    check_size(&tree, btn2, 80.0, 40.0);
    check_pos(&tree, logo, 0.0, 0.0);
    check_pos(&tree, spacer, 100.0, 0.0);
    check_pos(&tree, btn1, 640.0, 0.0);
    check_pos(&tree, btn2, 720.0, 0.0);
}

#[test]
fn test_mixed_all_sizing_modes() {
    // Test all sizing modes together: Fixed, Percentage, Fill
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(500.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let fixed = tree.add_node(
        "Fixed",
        Sizing::Fixed(100.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let pct = tree.add_node(
        "Pct",
        Sizing::Percentage(20.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let fill1 = tree.add_node("Fill1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fill2 = tree.add_node("Fill2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, fixed);
    tree.add_child(p, pct);
    tree.add_child(p, fill1);
    tree.add_child(p, fill2);

    tree.layout(p, [500.0, 100.0], [0.0, 0.0]);

    // Fixed: 100, Pct: 100 (20% of 500), Remaining: 300 split into 150 each
    check_size(&tree, fixed, 100.0, 50.0);
    check_size(&tree, pct, 100.0, 50.0);
    check_size(&tree, fill1, 150.0, 50.0);
    check_size(&tree, fill2, 150.0, 50.0);
    check_pos(&tree, fixed, 0.0, 0.0);
    check_pos(&tree, pct, 100.0, 0.0);
    check_pos(&tree, fill1, 200.0, 0.0);
    check_pos(&tree, fill2, 350.0, 0.0);
}

#[test]
fn test_grid_like_cells() {
    // Simulate a 2x2 grid using nested containers
    let mut tree = Tree::new();
    let root = tree.add_node(
        "Root",
        Sizing::Fixed(400.0),
        Sizing::Fixed(400.0),
        Direction::TopToBottom,
        0.0,
    );
    let row1 = tree.add_node(
        "Row1",
        Sizing::Fill,
        Sizing::Percentage(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let row2 = tree.add_node(
        "Row2",
        Sizing::Fill,
        Sizing::Percentage(50.0),
        Direction::LeftToRight,
        0.0,
    );

    let cell11 = tree.add_node(
        "Cell11",
        Sizing::Percentage(50.0),
        Sizing::Fill,
        Direction::LeftToRight,
        0.0,
    );
    let cell12 = tree.add_node(
        "Cell12",
        Sizing::Percentage(50.0),
        Sizing::Fill,
        Direction::LeftToRight,
        0.0,
    );
    let cell21 = tree.add_node(
        "Cell21",
        Sizing::Percentage(50.0),
        Sizing::Fill,
        Direction::LeftToRight,
        0.0,
    );
    let cell22 = tree.add_node(
        "Cell22",
        Sizing::Percentage(50.0),
        Sizing::Fill,
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(root, row1);
    tree.add_child(root, row2);
    tree.add_child(row1, cell11);
    tree.add_child(row1, cell12);
    tree.add_child(row2, cell21);
    tree.add_child(row2, cell22);

    tree.layout(root, [400.0, 400.0], [0.0, 0.0]);

    // Each cell should be 200x200
    check_size(&tree, cell11, 200.0, 200.0);
    check_size(&tree, cell12, 200.0, 200.0);
    check_size(&tree, cell21, 200.0, 200.0);
    check_size(&tree, cell22, 200.0, 200.0);
    check_pos(&tree, cell11, 0.0, 0.0);
    check_pos(&tree, cell12, 200.0, 0.0);
    check_pos(&tree, cell21, 0.0, 200.0);
    check_pos(&tree, cell22, 200.0, 200.0);
}

#[test]
fn test_empty_container() {
    // Container with no children
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    check_size(&tree, p, 100.0, 100.0);
    check_pos(&tree, p, 0.0, 0.0);
}

#[test]
fn test_single_fill_child() {
    // Single Fill child should take all available space
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let f = tree.add_node("F", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);

    tree.add_child(p, f);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_size(&tree, f, 200.0, 100.0);
    check_pos(&tree, f, 0.0, 0.0);
}

#[test]
fn test_percentage_with_fit_sibling() {
    // Mix of Percentage and Fit (both definite)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let pct = tree.add_node(
        "Pct",
        Sizing::Percentage(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fit_child = tree.add_node(
        "FitChild",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, pct);
    tree.add_child(p, fit);
    tree.add_child(fit, fit_child);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_size(&tree, pct, 100.0, 50.0);
    check_size(&tree, fit, 30.0, 50.0);
    check_pos(&tree, pct, 0.0, 0.0);
    check_pos(&tree, fit, 100.0, 0.0);
}

#[test]
fn test_rtl_with_percentages() {
    // Right-to-left layout with percentage sizing
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::RightToLeft,
        0.0,
    );
    let p1 = tree.add_node(
        "P1",
        Sizing::Percentage(25.0),
        Sizing::Fixed(50.0),
        Direction::RightToLeft,
        0.0,
    );
    let p2 = tree.add_node(
        "P2",
        Sizing::Percentage(50.0),
        Sizing::Fixed(50.0),
        Direction::RightToLeft,
        0.0,
    );
    let p3 = tree.add_node(
        "P3",
        Sizing::Percentage(25.0),
        Sizing::Fixed(50.0),
        Direction::RightToLeft,
        0.0,
    );

    tree.add_child(p, p1);
    tree.add_child(p, p2);
    tree.add_child(p, p3);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_size(&tree, p1, 50.0, 50.0);
    check_size(&tree, p2, 100.0, 50.0);
    check_size(&tree, p3, 50.0, 50.0);
    // RTL positioning: start from right
    check_pos(&tree, p1, 150.0, 0.0);
    check_pos(&tree, p2, 50.0, 0.0);
    check_pos(&tree, p3, 0.0, 0.0);
}

#[test]
fn test_btt_with_fill_and_fixed() {
    // Bottom-to-top with mixed sizing
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(300.0),
        Direction::BottomToTop,
        0.0,
    );
    let fixed = tree.add_node(
        "Fixed",
        Sizing::Fixed(50.0),
        Sizing::Fixed(100.0),
        Direction::BottomToTop,
        0.0,
    );
    let fill = tree.add_node("Fill", Sizing::Fixed(50.0), Sizing::Fill, Direction::BottomToTop, 0.0);
    let pct = tree.add_node(
        "Pct",
        Sizing::Fixed(50.0),
        Sizing::Percentage(20.0),
        Direction::BottomToTop,
        0.0,
    );

    tree.add_child(p, fixed);
    tree.add_child(p, fill);
    tree.add_child(p, pct);

    tree.layout(p, [100.0, 300.0], [0.0, 0.0]);

    // Fill gets: 300 - 100 - 60 = 140
    check_size(&tree, fixed, 50.0, 100.0);
    check_size(&tree, fill, 50.0, 140.0);
    check_size(&tree, pct, 50.0, 60.0);
    // BTT positioning: start from bottom
    check_pos(&tree, fixed, 0.0, 200.0);
    check_pos(&tree, fill, 0.0, 60.0);
    check_pos(&tree, pct, 0.0, 0.0);
}

// ========== NEW COMPREHENSIVE TEST CASES ==========

// Fit Container Edge Cases
#[test]
fn test_fit_with_no_children() {
    // Fit container with no children should be 0x0
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);

    tree.add_child(p, fit);

    tree.layout(p, [200.0, 200.0], [0.0, 0.0]);

    check_size(&tree, fit, 0.0, 0.0);
    check_pos(&tree, fit, 0.0, 0.0);
}

#[test]
fn test_fit_nested_within_fit() {
    // Multi-level Fit calculation
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(300.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit1 = tree.add_node("Fit1", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let fit2 = tree.add_node("Fit2", Sizing::Fit, Sizing::Fit, Direction::TopToBottom, 0.0);
    let r1 = tree.add_node(
        "R1",
        Sizing::Fixed(40.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );
    let r2 = tree.add_node(
        "R2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(20.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit1);
    tree.add_child(fit1, fit2);
    tree.add_child(fit2, r1);
    tree.add_child(fit2, r2);

    tree.layout(p, [300.0, 300.0], [0.0, 0.0]);

    // fit2 should be 40 wide (max of children) and 50 tall (sum of children)
    check_size(&tree, fit2, 40.0, 50.0);
    // fit1 should be 40 wide (sum in primary axis) and 50 tall (max in cross axis)
    check_size(&tree, fit1, 40.0, 50.0);
}

#[test]
fn test_fit_deeply_nested_fixed() {
    // Fit with deeply nested Fixed children (3+ levels)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(500.0),
        Sizing::Fixed(500.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let l1 = tree.add_node(
        "L1",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::TopToBottom,
        0.0,
    );
    let l2 = tree.add_node(
        "L2",
        Sizing::Fixed(80.0),
        Sizing::Fixed(80.0),
        Direction::LeftToRight,
        0.0,
    );
    let l3 = tree.add_node(
        "L3",
        Sizing::Fixed(60.0),
        Sizing::Fixed(60.0),
        Direction::TopToBottom,
        0.0,
    );
    let leaf = tree.add_node(
        "Leaf",
        Sizing::Fixed(40.0),
        Sizing::Fixed(40.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, l1);
    tree.add_child(l1, l2);
    tree.add_child(l2, l3);
    tree.add_child(l3, leaf);

    tree.layout(p, [500.0, 500.0], [0.0, 0.0]);

    check_size(&tree, fit, 100.0, 100.0);
    check_size(&tree, l1, 100.0, 100.0);
}

#[test]
fn test_fit_primary_fixed_cross() {
    // Fit in primary axis, Fixed in cross axis
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(20.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(25.0),
        Sizing::Fixed(20.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, c1);
    tree.add_child(fit, c2);

    tree.layout(p, [200.0, 200.0], [0.0, 0.0]);

    check_size(&tree, fit, 55.0, 50.0);
}

#[test]
fn test_fixed_primary_fit_cross() {
    // Fixed in primary axis, Fit in cross axis
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fixed(100.0), Sizing::Fit, Direction::LeftToRight, 0.0);
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(40.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(25.0),
        Sizing::Fixed(60.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, c1);
    tree.add_child(fit, c2);

    tree.layout(p, [200.0, 200.0], [0.0, 0.0]);

    check_size(&tree, fit, 100.0, 60.0);
}

// Zero and Boundary Cases
#[test]
fn test_zero_sized_parent() {
    // Zero-sized parent (0x0)
    let mut tree = Tree::new();
    let p = tree.add_node("P", Sizing::Fixed(0.0), Sizing::Fixed(0.0), Direction::LeftToRight, 0.0);
    let c = tree.add_node("C", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);

    tree.add_child(p, c);

    tree.layout(p, [0.0, 0.0], [0.0, 0.0]);

    check_size(&tree, p, 0.0, 0.0);
    check_size(&tree, c, 0.0, 0.0);
}

#[test]
fn test_zero_width_parent() {
    // Parent with 0 width but nonzero height
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(0.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c1 = tree.add_node("C1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let c2 = tree.add_node(
        "C2",
        Sizing::Percentage(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    tree.layout(p, [0.0, 100.0], [0.0, 0.0]);

    check_size(&tree, c1, 0.0, 50.0);
    check_size(&tree, c2, 0.0, 50.0);
}

#[test]
fn test_zero_percentage() {
    // 0% percentage sizing
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Percentage(0.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node("C2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_size(&tree, c1, 0.0, 50.0);
    check_size(&tree, c2, 200.0, 50.0);
}

#[test]
fn test_hundred_percent_single_child() {
    // Single 100% child
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Percentage(100.0),
        Sizing::Percentage(100.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_size(&tree, c, 200.0, 100.0);
    check_pos(&tree, c, 0.0, 0.0);
}

#[test]
fn test_very_small_sizes() {
    // Very small sizes (< 1.0) for floating-point precision
    let mut tree = Tree::new();
    let p = tree.add_node("P", Sizing::Fixed(0.5), Sizing::Fixed(0.3), Direction::LeftToRight, 0.0);
    let c1 = tree.add_node("C1", Sizing::Fixed(0.2), Sizing::Fill, Direction::LeftToRight, 0.0);
    let c2 = tree.add_node("C2", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    tree.layout(p, [0.5, 0.3], [0.0, 0.0]);

    check_size(&tree, c1, 0.2, 0.3);
    check_size(&tree, c2, 0.3, 0.3);
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 0.2, 0.0);
}

#[test]
fn test_many_fill_children_precision() {
    // Large number of Fill children (100+) to test floating-point accumulation
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(1000.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let mut children = Vec::new();

    for i in 0..150 {
        let c = tree.add_node(
            &format!("C{}", i),
            Sizing::Fill,
            Sizing::Fixed(50.0),
            Direction::LeftToRight,
            0.0,
        );
        children.push(c);
        tree.add_child(p, c);
    }

    tree.layout(p, [1000.0, 100.0], [0.0, 0.0]);

    // Each child should get 1000/150 = 6.666...
    let expected_size = 1000.0 / 150.0;
    for &c in &children {
        check_size(&tree, c, expected_size, 50.0);
    }

    // Check that positions accumulate correctly (last child should end at 1000.0)
    let last = children[149];
    let last_end = tree.nodes[last].pos[0] + tree.nodes[last].size[0];
    assert!(
        (last_end - 1000.0).abs() < 0.01,
        "Accumulated positions should equal parent size"
    );
}

// Missing Direction Coverage
#[test]
fn test_btt_with_percentages() {
    // Bottom-to-top with percentage sizing (missing test case)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(200.0),
        Direction::BottomToTop,
        0.0,
    );
    let p1 = tree.add_node(
        "P1",
        Sizing::Fixed(50.0),
        Sizing::Percentage(25.0),
        Direction::BottomToTop,
        0.0,
    );
    let p2 = tree.add_node(
        "P2",
        Sizing::Fixed(50.0),
        Sizing::Percentage(50.0),
        Direction::BottomToTop,
        0.0,
    );
    let p3 = tree.add_node(
        "P3",
        Sizing::Fixed(50.0),
        Sizing::Percentage(25.0),
        Direction::BottomToTop,
        0.0,
    );

    tree.add_child(p, p1);
    tree.add_child(p, p2);
    tree.add_child(p, p3);

    tree.layout(p, [100.0, 200.0], [0.0, 0.0]);

    check_size(&tree, p1, 50.0, 50.0);
    check_size(&tree, p2, 50.0, 100.0);
    check_size(&tree, p3, 50.0, 50.0);
    // BTT positioning: start from bottom
    check_pos(&tree, p1, 0.0, 150.0);
    check_pos(&tree, p2, 0.0, 50.0);
    check_pos(&tree, p3, 0.0, 0.0);
}

#[test]
fn test_cross_axis_fill_all_directions() {
    // Systematically test cross-axis Fill for all directions
    // Node sizing is always (width, height) - direction only affects layout of children

    // Test horizontal directions (primary=width, cross=height)
    for dir in [Direction::LeftToRight, Direction::RightToLeft] {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(200.0), Sizing::Fixed(100.0), dir, 0.0);
        let c = tree.add_node("C", Sizing::Fixed(50.0), Sizing::Fill, dir, 0.0);
        tree.add_child(p, c);
        tree.layout(p, [200.0, 100.0], [0.0, 0.0]);
        check_size(&tree, c, 50.0, 100.0); // width=Fixed, height=Fill(cross)
    }

    // Test vertical directions (primary=height, cross=width)
    for dir in [Direction::TopToBottom, Direction::BottomToTop] {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(200.0), Sizing::Fixed(100.0), dir, 0.0);
        let c = tree.add_node("C", Sizing::Fill, Sizing::Fixed(50.0), dir, 0.0);
        tree.add_child(p, c);
        tree.layout(p, [200.0, 100.0], [0.0, 0.0]);
        check_size(&tree, c, 200.0, 50.0); // width=Fill(cross), height=Fixed
    }
}

#[test]
fn test_cross_axis_percentage_ttb() {
    // Cross-axis Percentage in TopToBottom (previously only tested in LTR)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::TopToBottom,
        0.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Percentage(50.0),
        Sizing::Fixed(30.0),
        Direction::TopToBottom,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Percentage(75.0),
        Sizing::Fixed(30.0),
        Direction::TopToBottom,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_size(&tree, c1, 100.0, 30.0);
    check_size(&tree, c2, 150.0, 30.0);
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 0.0, 30.0);
}

// Realistic Edge Cases
#[test]
fn test_all_children_fill() {
    // All children are Fill (no Fixed/Percentage at all)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let f1 = tree.add_node("F1", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let f2 = tree.add_node("F2", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let f3 = tree.add_node("F3", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let f4 = tree.add_node("F4", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);

    tree.add_child(p, f1);
    tree.add_child(p, f2);
    tree.add_child(p, f3);
    tree.add_child(p, f4);

    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    check_size(&tree, f1, 75.0, 100.0);
    check_size(&tree, f2, 75.0, 100.0);
    check_size(&tree, f3, 75.0, 100.0);
    check_size(&tree, f4, 75.0, 100.0);
    check_pos(&tree, f1, 0.0, 0.0);
    check_pos(&tree, f2, 75.0, 0.0);
    check_pos(&tree, f3, 150.0, 0.0);
    check_pos(&tree, f4, 225.0, 0.0);
}

#[test]
fn test_single_percentage_child() {
    // Single Percentage child (common pattern not tested alone)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(400.0),
        Sizing::Fixed(300.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Percentage(60.0),
        Sizing::Percentage(40.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);

    tree.layout(p, [400.0, 300.0], [0.0, 0.0]);

    // Use tolerance for floating-point comparison
    let node = &tree.nodes[c];
    assert!(
        (node.size[0] - 240.0).abs() < 0.01,
        "{}: width {} != 240.0",
        node.name,
        node.size[0]
    );
    assert!(
        (node.size[1] - 120.0).abs() < 0.01,
        "{}: height {} != 120.0",
        node.name,
        node.size[1]
    );
    check_pos(&tree, c, 0.0, 0.0);
}

#[test]
fn test_oversubscribed_fixed() {
    // Fixed children sum > parent size (like percentage overflow)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let f1 = tree.add_node(
        "F1",
        Sizing::Fixed(60.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let f2 = tree.add_node(
        "F2",
        Sizing::Fixed(70.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let f3 = tree.add_node(
        "F3",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, f1);
    tree.add_child(p, f2);
    tree.add_child(p, f3);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    // Total = 180, exceeds parent 100 - children keep their sizes
    check_size(&tree, f1, 60.0, 50.0);
    check_size(&tree, f2, 70.0, 50.0);
    check_size(&tree, f3, 50.0, 50.0);
    check_pos(&tree, f1, 0.0, 0.0);
    check_pos(&tree, f2, 60.0, 0.0);
    check_pos(&tree, f3, 130.0, 0.0);
}

#[test]
fn test_fit_and_fill_siblings() {
    // Fit + Fill siblings (more complex interaction)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fit_child = tree.add_node(
        "FitChild",
        Sizing::Fixed(50.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );
    let fill1 = tree.add_node("Fill1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fill2 = tree.add_node("Fill2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fixed = tree.add_node(
        "Fixed",
        Sizing::Fixed(80.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, fit_child);
    tree.add_child(p, fill1);
    tree.add_child(p, fill2);
    tree.add_child(p, fixed);

    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    // Fit gets 50, Fixed gets 80, Fill children split remaining 170
    check_size(&tree, fit, 50.0, 50.0);
    check_size(&tree, fixed, 80.0, 50.0);
    check_size(&tree, fill1, 85.0, 50.0);
    check_size(&tree, fill2, 85.0, 50.0);
    check_pos(&tree, fit, 0.0, 0.0);
    check_pos(&tree, fill1, 50.0, 0.0);
    check_pos(&tree, fill2, 135.0, 0.0);
    check_pos(&tree, fixed, 220.0, 0.0);
}

// Re-layout and Mutation Tests
#[test]
fn test_relayout_different_parent_size() {
    // Re-layout with different parent sizes - state should reset properly
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Percentage(50.0),
        Sizing::Fill,
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node("C2", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    // First layout
    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);
    check_size(&tree, c1, 100.0, 100.0);
    check_size(&tree, c2, 100.0, 100.0);

    // Re-layout with different parent size
    tree.nodes[p].desired_size = [Sizing::Fixed(400.0), Sizing::Fixed(200.0)];
    tree.layout(p, [400.0, 200.0], [0.0, 0.0]);
    check_size(&tree, p, 400.0, 200.0);
    check_size(&tree, c1, 200.0, 200.0);
    check_size(&tree, c2, 200.0, 200.0);
}

#[test]
fn test_relayout_different_position() {
    // Re-layout with different parent position
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);

    // First layout at origin
    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);
    check_pos(&tree, p, 0.0, 0.0);
    check_pos(&tree, c, 0.0, 0.0);

    // Re-layout at different position
    tree.layout(p, [100.0, 100.0], [50.0, 75.0]);
    check_pos(&tree, p, 50.0, 75.0);
    check_pos(&tree, c, 50.0, 75.0);
}

#[test]
fn test_partial_tree_layout() {
    // Layout a subtree that's not the root
    let mut tree = Tree::new();
    let root = tree.add_node(
        "Root",
        Sizing::Fixed(500.0),
        Sizing::Fixed(500.0),
        Direction::LeftToRight,
        0.0,
    );
    let sub1 = tree.add_node(
        "Sub1",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::TopToBottom,
        0.0,
    );
    let sub2 = tree.add_node(
        "Sub2",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let leaf = tree.add_node("Leaf", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(root, sub1);
    tree.add_child(root, sub2);
    tree.add_child(sub2, leaf);

    // Layout just the sub2 subtree
    tree.layout(sub2, [100.0, 100.0], [300.0, 200.0]);

    check_size(&tree, sub2, 100.0, 100.0);
    check_pos(&tree, sub2, 300.0, 200.0);
    check_size(&tree, leaf, 100.0, 50.0);
    check_pos(&tree, leaf, 300.0, 200.0);
}

#[test]
fn test_mixed_fit_percentage_fixed_fill() {
    // Complex scenario: all sizing modes together with Fit
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(500.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let fixed = tree.add_node(
        "Fixed",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let pct = tree.add_node(
        "Pct",
        Sizing::Percentage(10.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fit_c1 = tree.add_node(
        "FitC1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit_c2 = tree.add_node(
        "FitC2",
        Sizing::Fixed(35.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );
    let fill1 = tree.add_node("Fill1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fill2 = tree.add_node("Fill2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, fixed);
    tree.add_child(p, pct);
    tree.add_child(p, fit);
    tree.add_child(fit, fit_c1);
    tree.add_child(fit, fit_c2);
    tree.add_child(p, fill1);
    tree.add_child(p, fill2);

    tree.layout(p, [500.0, 100.0], [0.0, 0.0]);

    // Fixed: 50, Pct: 50 (10% of 500), Fit: 65 (30+35), Fill children split 335
    check_size(&tree, fixed, 50.0, 50.0);
    check_size(&tree, pct, 50.0, 50.0);
    check_size(&tree, fit, 65.0, 50.0);
    check_size(&tree, fill1, 167.5, 50.0);
    check_size(&tree, fill2, 167.5, 50.0);
    check_pos(&tree, fixed, 0.0, 0.0);
    check_pos(&tree, pct, 50.0, 0.0);
    check_pos(&tree, fit, 100.0, 0.0);
    check_pos(&tree, fill1, 165.0, 0.0);
    check_pos(&tree, fill2, 332.5, 0.0);
}

// ========== ROBUSTNESS & EDGE CASE TESTS ==========

#[test]
fn test_single_fixed_child() {
    // Baseline: single Fixed child (important common case not explicitly tested)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Fixed(50.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);
    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_size(&tree, c, 50.0, 30.0);
    check_pos(&tree, c, 0.0, 0.0);
}

#[test]
fn test_negative_fixed_size() {
    // Negative sizes should work but may produce unexpected layouts
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Fixed(-50.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);
    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // Negative size is preserved (no clamping in current implementation)
    check_size(&tree, c, -50.0, 30.0);
    check_pos(&tree, c, 0.0, 0.0);
}

#[test]
fn test_negative_percentage() {
    // Negative percentage should produce negative sizes
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Percentage(-25.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);
    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_size(&tree, c, -50.0, 30.0);
    check_pos(&tree, c, 0.0, 0.0);
}

#[test]
fn test_infinity_size() {
    // Infinity sizes should propagate through layout
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(f32::INFINITY),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Percentage(50.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);
    tree.layout(p, [f32::INFINITY, 100.0], [0.0, 0.0]);

    assert_eq!(tree.nodes[p].size[0], f32::INFINITY);
    assert_eq!(tree.nodes[c].size[0], f32::INFINITY);
}

#[test]
fn test_nan_handling() {
    // NaN should propagate (NaN comparisons are always false)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Fixed(f32::NAN),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);
    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    assert!(tree.nodes[c].size[0].is_nan());
}

#[test]
fn test_all_fit_siblings() {
    // Multiple Fit siblings - should all calculate independently
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );

    let fit1 = tree.add_node("Fit1", Sizing::Fit, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fit1_c = tree.add_node(
        "Fit1C",
        Sizing::Fixed(40.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    let fit2 = tree.add_node("Fit2", Sizing::Fit, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fit2_c1 = tree.add_node(
        "Fit2C1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit2_c2 = tree.add_node(
        "Fit2C2",
        Sizing::Fixed(25.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    let fit3 = tree.add_node("Fit3", Sizing::Fit, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fit3_c = tree.add_node(
        "Fit3C",
        Sizing::Fixed(60.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit1);
    tree.add_child(fit1, fit1_c);
    tree.add_child(p, fit2);
    tree.add_child(fit2, fit2_c1);
    tree.add_child(fit2, fit2_c2);
    tree.add_child(p, fit3);
    tree.add_child(fit3, fit3_c);

    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    check_size(&tree, fit1, 40.0, 50.0);
    check_size(&tree, fit2, 55.0, 50.0);
    check_size(&tree, fit3, 60.0, 50.0);
    check_pos(&tree, fit1, 0.0, 0.0);
    check_pos(&tree, fit2, 40.0, 0.0);
    check_pos(&tree, fit3, 95.0, 0.0);
}

#[test]
fn test_deeply_nested_fill_chain() {
    // Fill propagates down through multiple levels
    let mut tree = Tree::new();
    let l1 = tree.add_node(
        "L1",
        Sizing::Fixed(1000.0),
        Sizing::Fixed(800.0),
        Direction::LeftToRight,
        0.0,
    );
    let l2 = tree.add_node("L2", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let l3 = tree.add_node("L3", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let l4 = tree.add_node("L4", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    let l5 = tree.add_node("L5", Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);

    tree.add_child(l1, l2);
    tree.add_child(l2, l3);
    tree.add_child(l3, l4);
    tree.add_child(l4, l5);

    tree.layout(l1, [1000.0, 800.0], [0.0, 0.0]);

    // All Fill nodes should get full parent size
    check_size(&tree, l2, 1000.0, 800.0);
    check_size(&tree, l3, 1000.0, 800.0);
    check_size(&tree, l4, 1000.0, 800.0);
    check_size(&tree, l5, 1000.0, 800.0);
}

#[test]
fn test_fit_with_mixed_direction_children() {
    // Fit container with children in different direction
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(300.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let v_child = tree.add_node(
        "VChild",
        Sizing::Fixed(50.0),
        Sizing::Fixed(80.0),
        Direction::TopToBottom,
        0.0,
    );
    let h_child = tree.add_node(
        "HChild",
        Sizing::Fixed(60.0),
        Sizing::Fixed(70.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, v_child);
    tree.add_child(fit, h_child);

    tree.layout(p, [300.0, 300.0], [0.0, 0.0]);

    // Fit should sum primary axis (50+60=110) and max cross axis (max(80,70)=80)
    check_size(&tree, fit, 110.0, 80.0);
}

#[test]
fn test_fractional_percentages() {
    // Percentages that don't divide evenly - test floating-point precision
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let p1 = tree.add_node(
        "P1",
        Sizing::Percentage(33.333),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let p2 = tree.add_node(
        "P2",
        Sizing::Percentage(33.333),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let p3 = tree.add_node(
        "P3",
        Sizing::Percentage(33.334),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, p1);
    tree.add_child(p, p2);
    tree.add_child(p, p3);

    tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

    // Check with tolerance for floating-point
    let node1 = &tree.nodes[p1];
    let node2 = &tree.nodes[p2];
    let node3 = &tree.nodes[p3];
    assert!((node1.size[0] - 33.333).abs() < 0.01, "P1 width mismatch");
    assert!((node2.size[0] - 33.333).abs() < 0.01, "P2 width mismatch");
    assert!((node3.size[0] - 33.334).abs() < 0.01, "P3 width mismatch");
}

#[test]
fn test_very_large_sizes() {
    // Test numerical stability with very large values
    let mut tree = Tree::new();
    let p = tree.add_node("P", Sizing::Fixed(1e8), Sizing::Fixed(1e8), Direction::LeftToRight, 0.0);
    let c1 = tree.add_node(
        "C1",
        Sizing::Percentage(25.0),
        Sizing::Fixed(1e7),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node("C2", Sizing::Fill, Sizing::Fixed(1e7), Direction::LeftToRight, 0.0);

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    tree.layout(p, [1e8, 1e8], [0.0, 0.0]);

    let node_c1 = &tree.nodes[c1];
    let node_c2 = &tree.nodes[c2];
    assert!((node_c1.size[0] - 2.5e7).abs() < 1000.0, "C1 size incorrect");
    assert!((node_c2.size[0] - 7.5e7).abs() < 1000.0, "C2 size incorrect");
}

#[test]
fn test_fit_with_cross_axis_fill_children() {
    // Fit in both axes with children that have Fill in cross-axis
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(200.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(40.0),
        Sizing::Fixed(60.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(35.0),
        Sizing::Fixed(80.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, c1);
    tree.add_child(fit, c2);

    tree.layout(p, [300.0, 200.0], [0.0, 0.0]);

    // Fit should sum widths (40+35=75) and max heights (max(60,80)=80)
    check_size(&tree, fit, 75.0, 80.0);
    // Children should NOT fill cross-axis since they have Fixed heights
    check_size(&tree, c1, 40.0, 60.0);
    check_size(&tree, c2, 35.0, 80.0);
}

#[test]
fn test_layout_parent_size_mismatch() {
    // What happens when parent_size doesn't match node's Fixed size?
    // In current implementation, percentage calculations use parent_size parameter
    let mut tree = Tree::new();
    let root = tree.add_node(
        "Root",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Percentage(50.0),
        Sizing::Percentage(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(root, c);

    // Pass different parent_size than root's Fixed size
    tree.layout(root, [200.0, 200.0], [0.0, 0.0]);

    // Node's desired_size (Fixed) takes precedence for itself
    check_size(&tree, root, 100.0, 100.0);
    // But child calculates percentage from the passed parent_size (200x200)
    check_size(&tree, c, 50.0, 50.0); // 50% of root's actual 100x100
}

#[test]
fn test_single_child_percentage_over_100() {
    // Single child with percentage > 100%
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree.add_node(
        "C",
        Sizing::Percentage(150.0),
        Sizing::Percentage(120.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c);
    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // Child overflows parent - use tolerance for floating-point
    let node_c = &tree.nodes[c];
    assert!((node_c.size[0] - 300.0).abs() < 0.01, "C width incorrect");
    assert!((node_c.size[1] - 120.0).abs() < 0.01, "C height incorrect");
    check_pos(&tree, c, 0.0, 0.0);
}

#[test]
fn test_add_child_and_relayout() {
    // Mutate tree between layouts by adding children
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c1 = tree.add_node("C1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, c1);
    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    // Initially, c1 gets all space
    check_size(&tree, c1, 300.0, 50.0);

    // Add another child
    let c2 = tree.add_node("C2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    tree.add_child(p, c2);
    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    // Now space should be split
    check_size(&tree, c1, 150.0, 50.0);
    check_size(&tree, c2, 150.0, 50.0);
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 150.0, 0.0);
}

#[test]
fn test_child_order_matters() {
    // Same children, different order - positioning should differ
    let mut tree1 = Tree::new();
    let p1 = tree1.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let a = tree1.add_node(
        "A",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let b = tree1.add_node(
        "B",
        Sizing::Fixed(100.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c = tree1.add_node("C", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree1.add_child(p1, a);
    tree1.add_child(p1, b);
    tree1.add_child(p1, c);
    tree1.layout(p1, [300.0, 100.0], [0.0, 0.0]);

    let pos_a_1 = tree1.nodes[a].pos[0];
    let pos_b_1 = tree1.nodes[b].pos[0];
    let pos_c_1 = tree1.nodes[c].pos[0];

    // Different order
    let mut tree2 = Tree::new();
    let p2 = tree2.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree2.add_node("C", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let a2 = tree2.add_node(
        "A",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let b2 = tree2.add_node(
        "B",
        Sizing::Fixed(100.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree2.add_child(p2, c2);
    tree2.add_child(p2, a2);
    tree2.add_child(p2, b2);
    tree2.layout(p2, [300.0, 100.0], [0.0, 0.0]);

    let pos_c_2 = tree2.nodes[c2].pos[0];
    let pos_a_2 = tree2.nodes[a2].pos[0];
    let pos_b_2 = tree2.nodes[b2].pos[0];

    // Positions should be different due to order
    assert_ne!(pos_a_1, pos_a_2, "A position should differ");
    assert_ne!(pos_b_1, pos_b_2, "B position should differ");
    assert_ne!(pos_c_1, pos_c_2, "C position should differ");

    // Verify tree2 order: C(Fill=150), A(50), B(100)
    check_pos(&tree2, c2, 0.0, 0.0);
    check_pos(&tree2, a2, 150.0, 0.0);
    check_pos(&tree2, b2, 200.0, 0.0);
}

#[test]
fn test_rtl_with_fit() {
    // Right-to-left with Fit sizing
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::RightToLeft,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fit_c1 = tree.add_node(
        "FitC1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit_c2 = tree.add_node(
        "FitC2",
        Sizing::Fixed(40.0),
        Sizing::Fixed(30.0),
        Direction::LeftToRight,
        0.0,
    );
    let fixed = tree.add_node(
        "Fixed",
        Sizing::Fixed(80.0),
        Sizing::Fixed(50.0),
        Direction::RightToLeft,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, fit_c1);
    tree.add_child(fit, fit_c2);
    tree.add_child(p, fixed);

    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    check_size(&tree, fit, 70.0, 50.0);
    check_size(&tree, fixed, 80.0, 50.0);
    // RTL: fit at right, then fixed
    check_pos(&tree, fit, 230.0, 0.0);
    check_pos(&tree, fixed, 150.0, 0.0);
}

#[test]
fn test_btt_with_fit() {
    // Bottom-to-top with Fit sizing
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(300.0),
        Direction::BottomToTop,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fixed(50.0), Sizing::Fit, Direction::TopToBottom, 0.0);
    let fit_c1 = tree.add_node(
        "FitC1",
        Sizing::Fixed(30.0),
        Sizing::Fixed(30.0),
        Direction::TopToBottom,
        0.0,
    );
    let fit_c2 = tree.add_node(
        "FitC2",
        Sizing::Fixed(30.0),
        Sizing::Fixed(40.0),
        Direction::TopToBottom,
        0.0,
    );
    let fixed = tree.add_node(
        "Fixed",
        Sizing::Fixed(50.0),
        Sizing::Fixed(80.0),
        Direction::BottomToTop,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, fit_c1);
    tree.add_child(fit, fit_c2);
    tree.add_child(p, fixed);

    tree.layout(p, [100.0, 300.0], [0.0, 0.0]);

    check_size(&tree, fit, 50.0, 70.0);
    check_size(&tree, fixed, 50.0, 80.0);
    // BTT: fit at bottom, then fixed
    check_pos(&tree, fit, 0.0, 230.0);
    check_pos(&tree, fixed, 0.0, 150.0);
}

#[test]
fn test_fit_with_single_child() {
    // Fit with single child (baseline for Fit behavior)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(200.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit = tree.add_node("Fit", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let c = tree.add_node(
        "C",
        Sizing::Fixed(75.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit);
    tree.add_child(fit, c);

    tree.layout(p, [200.0, 200.0], [0.0, 0.0]);

    check_size(&tree, fit, 75.0, 50.0);
    check_size(&tree, c, 75.0, 50.0);
}

#[test]
fn test_zero_percentage_with_fill() {
    // 0% child alongside Fill child
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let zero_pct = tree.add_node(
        "Zero",
        Sizing::Percentage(0.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let fill = tree.add_node("Fill", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let fixed = tree.add_node(
        "Fixed",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, zero_pct);
    tree.add_child(p, fill);
    tree.add_child(p, fixed);

    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    check_size(&tree, zero_pct, 0.0, 50.0);
    check_size(&tree, fill, 250.0, 50.0); // 300 - 0 - 50
    check_size(&tree, fixed, 50.0, 50.0);
}

#[test]
fn test_hundred_percent_multiple_children() {
    // Multiple 100% children (each takes full parent width, causing massive overflow)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Percentage(100.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Percentage(100.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // Both get 100% = 200, total 400 (overflow)
    check_size(&tree, c1, 200.0, 50.0);
    check_size(&tree, c2, 200.0, 50.0);
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 200.0, 0.0);
}

#[test]
fn test_alternating_directions_deeply_nested() {
    // Deep nesting with alternating directions (H->V->H->V->H)
    let mut tree = Tree::new();
    let l1 = tree.add_node(
        "L1",
        Sizing::Fixed(500.0),
        Sizing::Fixed(500.0),
        Direction::LeftToRight,
        0.0,
    );
    let l2 = tree.add_node(
        "L2",
        Sizing::Percentage(80.0),
        Sizing::Percentage(80.0),
        Direction::TopToBottom,
        0.0,
    );
    let l3 = tree.add_node(
        "L3",
        Sizing::Percentage(80.0),
        Sizing::Percentage(80.0),
        Direction::LeftToRight,
        0.0,
    );
    let l4 = tree.add_node(
        "L4",
        Sizing::Percentage(80.0),
        Sizing::Percentage(80.0),
        Direction::TopToBottom,
        0.0,
    );
    let l5 = tree.add_node(
        "L5",
        Sizing::Percentage(80.0),
        Sizing::Percentage(80.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(l1, l2);
    tree.add_child(l2, l3);
    tree.add_child(l3, l4);
    tree.add_child(l4, l5);

    tree.layout(l1, [500.0, 500.0], [0.0, 0.0]);

    // Each level is 80% of previous: 500, 400, 320, 256, 204.8
    check_size(&tree, l2, 400.0, 400.0);
    check_size(&tree, l3, 320.0, 320.0);
    check_size(&tree, l4, 256.0, 256.0);
    let node_l5 = &tree.nodes[l5];
    assert!((node_l5.size[0] - 204.8).abs() < 0.1, "L5 width incorrect");
    assert!((node_l5.size[1] - 204.8).abs() < 0.1, "L5 height incorrect");
}

#[test]
fn test_many_nested_fit_containers() {
    // Deep chain of Fit containers
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(500.0),
        Sizing::Fixed(500.0),
        Direction::LeftToRight,
        0.0,
    );
    let fit1 = tree.add_node("Fit1", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let fit2 = tree.add_node("Fit2", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let fit3 = tree.add_node("Fit3", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let fit4 = tree.add_node("Fit4", Sizing::Fit, Sizing::Fit, Direction::LeftToRight, 0.0);
    let leaf = tree.add_node(
        "Leaf",
        Sizing::Fixed(100.0),
        Sizing::Fixed(80.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, fit1);
    tree.add_child(fit1, fit2);
    tree.add_child(fit2, fit3);
    tree.add_child(fit3, fit4);
    tree.add_child(fit4, leaf);

    tree.layout(p, [500.0, 500.0], [0.0, 0.0]);

    // All Fit containers should match leaf size
    check_size(&tree, fit1, 100.0, 80.0);
    check_size(&tree, fit2, 100.0, 80.0);
    check_size(&tree, fit3, 100.0, 80.0);
    check_size(&tree, fit4, 100.0, 80.0);
    check_size(&tree, leaf, 100.0, 80.0);
}

#[test]
fn test_fill_in_fit_container_sibling() {
    // Fill child alongside Fixed child - Fill should take leftover space
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(100.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node("C2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let c3 = tree.add_node(
        "C3",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);
    tree.add_child(p, c3);

    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    check_size(&tree, c1, 100.0, 50.0);
    check_size(&tree, c2, 150.0, 50.0); // 300 - 100 - 50
    check_size(&tree, c3, 50.0, 50.0);
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 100.0, 0.0);
    check_pos(&tree, c3, 250.0, 0.0);
}

// ========== GAP TESTS ==========

#[test]
fn test_gap_basic_horizontal() {
    // Basic gap between fixed-size children in horizontal layout
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        10.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(60.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c3 = tree.add_node(
        "C3",
        Sizing::Fixed(40.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);
    tree.add_child(p, c3);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // With gap=10, positions should be: 0, 50+10=60, 60+60+10=130
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 60.0, 0.0);
    check_pos(&tree, c3, 130.0, 0.0);
}

#[test]
fn test_gap_basic_vertical() {
    // Basic gap between fixed-size children in vertical layout
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(300.0),
        Direction::TopToBottom,
        15.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(50.0),
        Sizing::Fixed(80.0),
        Direction::TopToBottom,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(50.0),
        Sizing::Fixed(90.0),
        Direction::TopToBottom,
        0.0,
    );
    let c3 = tree.add_node(
        "C3",
        Sizing::Fixed(50.0),
        Sizing::Fixed(70.0),
        Direction::TopToBottom,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);
    tree.add_child(p, c3);

    tree.layout(p, [100.0, 300.0], [0.0, 0.0]);

    // With gap=15, y positions: 0, 80+15=95, 95+90+15=200
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 0.0, 95.0);
    check_pos(&tree, c3, 0.0, 200.0);
}

#[test]
fn test_gap_with_fill_children() {
    // Gap should reduce available space for Fill children
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        20.0,
    );
    let c1 = tree.add_node("C1", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let c2 = tree.add_node("C2", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
    let c3 = tree.add_node("C3", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, c1);
    tree.add_child(p, c2);
    tree.add_child(p, c3);

    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    // Total gap space: 20 * 2 = 40
    // Available for Fill: 300 - 40 = 260, divided by 3 = 86.666...
    let expected_width = 260.0 / 3.0;
    check_size(&tree, c1, expected_width, 50.0);
    check_size(&tree, c2, expected_width, 50.0);
    check_size(&tree, c3, expected_width, 50.0);
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, expected_width + 20.0, 0.0);
    check_pos(&tree, c3, expected_width * 2.0 + 40.0, 0.0);
}

#[test]
fn test_gap_with_mixed_sizing() {
    // Gap with Fixed, Percentage, and Fill children
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(500.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        10.0,
    );
    let fixed = tree.add_node(
        "Fixed",
        Sizing::Fixed(100.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let pct = tree.add_node(
        "Pct",
        Sizing::Percentage(20.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let fill = tree.add_node("Fill", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, fixed);
    tree.add_child(p, pct);
    tree.add_child(p, fill);

    tree.layout(p, [500.0, 100.0], [0.0, 0.0]);

    // Fixed: 100, Pct: 100 (20% of 500), Gap: 2*10=20
    // Fill gets: 500 - 100 - 100 - 20 = 280
    check_size(&tree, fixed, 100.0, 50.0);
    check_size(&tree, pct, 100.0, 50.0);
    check_size(&tree, fill, 280.0, 50.0);
    check_pos(&tree, fixed, 0.0, 0.0);
    check_pos(&tree, pct, 110.0, 0.0);
    check_pos(&tree, fill, 220.0, 0.0);
}

#[test]
fn test_gap_right_to_left() {
    // Gap should work correctly in reversed directions
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::RightToLeft,
        10.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::RightToLeft,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(60.0),
        Sizing::Fixed(50.0),
        Direction::RightToLeft,
        0.0,
    );
    let c3 = tree.add_node(
        "C3",
        Sizing::Fixed(40.0),
        Sizing::Fixed(50.0),
        Direction::RightToLeft,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);
    tree.add_child(p, c3);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // RTL with gap=10: c1 at 150, c2 at 80, c3 at 30
    check_pos(&tree, c1, 150.0, 0.0);
    check_pos(&tree, c2, 80.0, 0.0);
    check_pos(&tree, c3, 30.0, 0.0);
}

#[test]
fn test_gap_bottom_to_top() {
    // Gap in bottom-to-top layout
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(100.0),
        Sizing::Fixed(300.0),
        Direction::BottomToTop,
        15.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(50.0),
        Sizing::Fixed(80.0),
        Direction::BottomToTop,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(50.0),
        Sizing::Fixed(90.0),
        Direction::BottomToTop,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    tree.layout(p, [100.0, 300.0], [0.0, 0.0]);

    // BTT with gap=15: c1 at 220, c2 at 115
    check_pos(&tree, c1, 0.0, 220.0);
    check_pos(&tree, c2, 0.0, 115.0);
}

#[test]
fn test_gap_single_child() {
    // Gap should have no effect with single child (no gaps to add)
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        50.0,
    );
    let c = tree.add_node("C", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);

    tree.add_child(p, c);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    // With only 1 child, no gap is applied, so Fill gets full width
    check_size(&tree, c, 200.0, 50.0);
    check_pos(&tree, c, 0.0, 0.0);
}

#[test]
fn test_gap_zero() {
    // Zero gap should behave like no gap
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(200.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        0.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(50.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );
    let c2 = tree.add_node(
        "C2",
        Sizing::Fixed(60.0),
        Sizing::Fixed(50.0),
        Direction::LeftToRight,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);

    tree.layout(p, [200.0, 100.0], [0.0, 0.0]);

    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 50.0, 0.0); // No gap
}

#[test]
fn test_gap_nested_containers() {
    // Gap should be independent per container
    let mut tree = Tree::new();
    let p = tree.add_node(
        "P",
        Sizing::Fixed(300.0),
        Sizing::Fixed(100.0),
        Direction::LeftToRight,
        20.0,
    );
    let c1 = tree.add_node(
        "C1",
        Sizing::Fixed(100.0),
        Sizing::Fixed(100.0),
        Direction::TopToBottom,
        10.0,
    );
    let c2 = tree.add_node("C2", Sizing::Fill, Sizing::Fixed(100.0), Direction::LeftToRight, 0.0);

    let gc1 = tree.add_node(
        "GC1",
        Sizing::Fixed(80.0),
        Sizing::Fixed(30.0),
        Direction::TopToBottom,
        0.0,
    );
    let gc2 = tree.add_node(
        "GC2",
        Sizing::Fixed(80.0),
        Sizing::Fixed(40.0),
        Direction::TopToBottom,
        0.0,
    );

    tree.add_child(p, c1);
    tree.add_child(p, c2);
    tree.add_child(c1, gc1);
    tree.add_child(c1, gc2);

    tree.layout(p, [300.0, 100.0], [0.0, 0.0]);

    // Parent gap=20 between c1 and c2
    check_pos(&tree, c1, 0.0, 0.0);
    check_pos(&tree, c2, 120.0, 0.0);
    // c1's internal gap=10 between grandchildren
    check_pos(&tree, gc1, 0.0, 0.0);
    check_pos(&tree, gc2, 0.0, 40.0); // 30 + 10
}
