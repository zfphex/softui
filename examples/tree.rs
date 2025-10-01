#![allow(unused)]
use softui::tree::*;

fn benchmark<F: FnMut()>(name: &str, iterations: usize, mut f: F) {
    // Warmup
    for _ in 0..10 {
        f();
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();

    let avg_ns = elapsed.as_nanos() / iterations as u128;
    let avg_us = avg_ns as f64 / 1000.0;

    println!(
        "{:40} {:>8} iterations  {:>10.2} μs/iter  ({:>6.2} ns/iter)",
        name, iterations, avg_us, avg_ns
    );
}

fn main() {
    println!("Running layout benchmarks...\n");
    println!("Measuring ONLY layout performance (tree construction excluded)\n");

    // Benchmark 1: Shallow tree with many children
    let mut tree1 = Tree::new();
    let p1 = tree1.add_node(
        Sizing::Fixed(1000.0),
        Sizing::Fixed(1000.0),
        Direction::LeftToRight,
        0.0,
    );
    for i in 0..100 {
        let child = tree1.add_node(Sizing::Fill, Sizing::Fixed(10.0), Direction::LeftToRight, 0.0);
        tree1.add_child(p1, child);
    }
    benchmark("Shallow (100 children)", 10_000, || {
        tree1.layout(p1, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree1);
    });

    // Benchmark 2: Deep nested tree (10 levels)
    let mut tree2 = Tree::new();
    let mut current = tree2.add_node(
        Sizing::Fixed(1000.0),
        Sizing::Fixed(1000.0),
        Direction::LeftToRight,
        0.0,
    );
    for i in 1..10 {
        let child = tree2.add_node(
            Sizing::Percentage(90.0),
            Sizing::Percentage(90.0),
            Direction::LeftToRight,
            0.0,
        );
        tree2.add_child(current, child);
        current = child;
    }
    benchmark("Deep nested (10 levels)", 10_000, || {
        tree2.layout(0, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree2);
    });

    // Benchmark 3: Balanced tree (4 levels, 3 children each)
    fn build_tree(tree: &mut Tree, depth: usize, parent: usize) {
        if depth == 0 {
            return;
        }
        for i in 0..3 {
            let child = tree.add_node(Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
            tree.add_child(parent, child);
            build_tree(tree, depth - 1, child);
        }
    }
    let mut tree3 = Tree::new();
    let root3 = tree3.add_node(
        Sizing::Fixed(1000.0),
        Sizing::Fixed(1000.0),
        Direction::LeftToRight,
        0.0,
    );
    build_tree(&mut tree3, 4, root3);
    benchmark("Balanced tree (4x3)", 10_000, || {
        tree3.layout(root3, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree3);
    });

    // Benchmark 4: Mixed sizing modes
    let mut tree4 = Tree::new();
    let p4 = tree4.add_node(
        Sizing::Fixed(1000.0),
        Sizing::Fixed(1000.0),
        Direction::LeftToRight,
        0.0,
    );
    for i in 0..20 {
        let sizing = match i % 4 {
            0 => Sizing::Fixed(50.0),
            1 => Sizing::Fill,
            2 => Sizing::Percentage(5.0),
            _ => Sizing::Fit,
        };
        let child = tree4.add_node(sizing, Sizing::Fixed(10.0), Direction::LeftToRight, 0.0);
        tree4.add_child(p4, child);

        if sizing == Sizing::Fit {
            // Add a child for Fit sizing
            let grandchild = tree4.add_node(Sizing::Fixed(10.0), Sizing::Fixed(10.0), Direction::LeftToRight, 0.0);
            tree4.add_child(child, grandchild);
        }
    }
    benchmark("Mixed sizing modes", 10_000, || {
        tree4.layout(p4, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree4);
    });

    // Benchmark 5: Wide and deep (realistic UI)
    let mut tree5 = Tree::new();
    let root5 = tree5.add_node(
        Sizing::Fixed(1920.0),
        Sizing::Fixed(1080.0),
        Direction::TopToBottom,
        0.0,
    );
    // Header
    let header = tree5.add_node(Sizing::Fill, Sizing::Fixed(60.0), Direction::LeftToRight, 0.0);
    tree5.add_child(root5, header);
    for i in 0..5 {
        let btn = tree5.add_node(Sizing::Fixed(100.0), Sizing::Fill, Direction::LeftToRight, 0.0);
        tree5.add_child(header, btn);
    }
    // Content area
    let content = tree5.add_node(Sizing::Fill, Sizing::Fill, Direction::LeftToRight, 0.0);
    tree5.add_child(root5, content);
    // Sidebar
    let sidebar = tree5.add_node(Sizing::Fixed(250.0), Sizing::Fill, Direction::TopToBottom, 0.0);
    tree5.add_child(content, sidebar);
    for i in 0..10 {
        let item = tree5.add_node(Sizing::Fill, Sizing::Fixed(40.0), Direction::LeftToRight, 0.0);
        tree5.add_child(sidebar, item);
    }
    // Main content
    let main = tree5.add_node(Sizing::Fill, Sizing::Fill, Direction::TopToBottom, 0.0);
    tree5.add_child(content, main);
    for i in 0..20 {
        let row = tree5.add_node(Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight, 0.0);
        tree5.add_child(main, row);
    }
    benchmark("Realistic UI layout", 10_000, || {
        tree5.layout(root5, [1920.0, 1080.0], [0.0, 0.0]);
        std::hint::black_box(&tree5);
    });

    // Benchmark 6: Stress test (1000 nodes)
    let mut tree6 = Tree::new();
    let root6 = tree6.add_node(
        Sizing::Fixed(1000.0),
        Sizing::Fixed(1000.0),
        Direction::LeftToRight,
        0.0,
    );
    // Create 10 columns with 100 items each
    for col in 0..10 {
        let column = tree6.add_node(Sizing::Fill, Sizing::Fill, Direction::TopToBottom, 0.0);
        tree6.add_child(root6, column);

        for row in 0..100 {
            let item = tree6.add_node(Sizing::Fill, Sizing::Fixed(10.0), Direction::LeftToRight, 0.0);
            tree6.add_child(column, item);
        }
    }
    benchmark("Stress test (1000 nodes)", 10_000, || {
        tree6.layout(root6, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree6);
    });

    println!("\n=== Benchmark complete! ===");
}
