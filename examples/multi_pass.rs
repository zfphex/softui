#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sizing {
    Fixed(f32),
    Percentage(f32),
    Fill,
    Fit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl Direction {
    fn axis(&self) -> usize {
        match self {
            Direction::LeftToRight | Direction::RightToLeft => 0, // X
            Direction::TopToBottom | Direction::BottomToTop => 1, // Y
        }
    }

    fn reversed(&self) -> bool {
        matches!(self, Direction::RightToLeft | Direction::BottomToTop)
    }
}

#[derive(Debug, Clone)]
struct Container {
    name: String,
    desired_size: [Sizing; 2],
    direction: Direction,
    size: [f32; 2],
    pos: [f32; 2],
    children: Vec<Container>,
}

impl Container {
    fn new(name: &str, width: Sizing, height: Sizing) -> Self {
        Self {
            name: name.to_string(),
            desired_size: [width, height],
            direction: Direction::LeftToRight,
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            children: vec![],
        }
    }

    fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    fn with_children(mut self, children: Vec<Container>) -> Self {
        self.children = children;
        self
    }
}

fn calculate_layout(container: &mut Container, parent_size: [f32; 2]) {
    for axis in 0..2 {
        container.size[axis] = match container.desired_size[axis] {
            Sizing::Fixed(v) => v,
            Sizing::Percentage(p) => parent_size[axis] * (p / 100.0),
            Sizing::Fill => parent_size[axis],
            Sizing::Fit => calculate_fit_size(container, axis),
        };
    }
    layout_children(container);
}

#[rustfmt::skip] 
fn calculate_fit_size(container: &Container, axis: usize) -> f32 {
    let primary = container.direction.axis();
    let sum_mode = axis == primary;
    
    let mut result = 0.0;
    for child in &container.children {
        let child_size = match child.desired_size[axis] {
            Sizing::Fixed(value) => value,
            Sizing::Fit => calculate_fit_size(child, axis),
            Sizing::Percentage(_) | Sizing::Fill => {
                panic!("Fit containers cannot have Percentage or Fill children");
            }
        };
        result = if sum_mode { result + child_size } else { result.max(child_size) };
    }
    result
}

fn layout_children(parent: &mut Container) {
    if parent.children.is_empty() {
        return;
    }

    let primary = parent.direction.axis();
    let cross = 1 - primary;

    // Pass 1: measure
    let mut used_primary = 0.0;
    let mut fill_indices: Vec<usize> = Vec::new();

    for (i, child) in parent.children.iter_mut().enumerate() {
        // cross axis
        child.size[cross] = match child.desired_size[cross] {
            Sizing::Fixed(v) => v,
            Sizing::Percentage(p) => parent.size[cross] * (p / 100.0),
            Sizing::Fill => parent.size[cross],
            Sizing::Fit => calculate_fit_size(child, cross),
        };

        // primary axis
        match child.desired_size[primary] {
            Sizing::Fixed(v) => {
                child.size[primary] = v;
                used_primary += v;
            }
            Sizing::Percentage(p) => {
                let s = parent.size[primary] * (p / 100.0);
                child.size[primary] = s;
                used_primary += s;
            }
            Sizing::Fit => {
                let s = calculate_fit_size(child, primary);
                child.size[primary] = s;
                used_primary += s;
            }
            Sizing::Fill => {
                fill_indices.push(i);
            }
        }
    }

    // Pass 2: fill distribution
    if !fill_indices.is_empty() {
        let remaining = (parent.size[primary] - used_primary).max(0.0);
        let per_fill = remaining / (fill_indices.len() as f32);
        for &idx in &fill_indices {
            parent.children[idx].size[primary] = per_fill;
        }
    }

    // Pass 3: positioning
    let mut offset = if parent.direction.reversed() {
        parent.size[primary]
    } else {
        0.0
    };

    for child in parent.children.iter_mut() {
        if parent.direction.reversed() {
            offset -= child.size[primary];
        }
        child.pos[primary] = offset;
        child.pos[cross] = 0.0;
        if !parent.direction.reversed() {
            offset += child.size[primary];
        }
    }

    for child in parent.children.iter_mut() {
        layout_children(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_size(c: &Container, w: f32, h: f32) {
        assert_eq!(c.size[0], w, "{}: width {} != {}", c.name, c.size[0], w);
        assert_eq!(c.size[1], h, "{}: height {} != {}", c.name, c.size[1], h);
    }

    fn check_pos(c: &Container, x: f32, y: f32) {
        assert_eq!(c.pos[0], x, "{}: x {} != {}", c.name, c.pos[0], x);
        assert_eq!(c.pos[1], y, "{}: y {} != {}", c.name, c.pos[1], y);
    }

    #[test]
    fn test_left_to_right() {
        let mut parent = Container::new("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0))
            .with_direction(Direction::LeftToRight)
            .with_children(vec![
                Container::new("B1", Sizing::Fixed(30.0), Sizing::Fixed(30.0)),
                Container::new("B2", Sizing::Fixed(40.0), Sizing::Fixed(40.0)),
                Container::new("B3", Sizing::Fill, Sizing::Fixed(50.0)),
            ]);

        calculate_layout(&mut parent, [100.0, 100.0]);

        check_size(&parent.children[0], 30.0, 30.0);
        check_pos(&parent.children[0], 0.0, 0.0);
        check_size(&parent.children[1], 40.0, 40.0);
        check_pos(&parent.children[1], 30.0, 0.0);
        check_size(&parent.children[2], 30.0, 50.0);
        check_pos(&parent.children[2], 70.0, 0.0);
    }

    #[test]
    fn test_right_to_left() {
        let mut parent = Container::new("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0))
            .with_direction(Direction::RightToLeft)
            .with_children(vec![
                Container::new("B1", Sizing::Fixed(30.0), Sizing::Fixed(30.0)),
                Container::new("B2", Sizing::Fixed(40.0), Sizing::Fixed(40.0)),
                Container::new("B3", Sizing::Fill, Sizing::Fixed(50.0)),
            ]);

        calculate_layout(&mut parent, [100.0, 100.0]);

        check_size(&parent.children[0], 30.0, 30.0);
        check_pos(&parent.children[0], 70.0, 0.0);
        check_size(&parent.children[1], 40.0, 40.0);
        check_pos(&parent.children[1], 30.0, 0.0);
        check_size(&parent.children[2], 30.0, 50.0);
        check_pos(&parent.children[2], 0.0, 0.0);
    }

    #[test]
    fn test_top_to_bottom() {
        let mut parent = Container::new("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0))
            .with_direction(Direction::TopToBottom)
            .with_children(vec![
                Container::new("B1", Sizing::Fixed(30.0), Sizing::Fixed(30.0)),
                Container::new("B2", Sizing::Fixed(40.0), Sizing::Fixed(40.0)),
                Container::new("B3", Sizing::Fixed(50.0), Sizing::Fill),
            ]);

        calculate_layout(&mut parent, [100.0, 100.0]);

        check_size(&parent.children[0], 30.0, 30.0);
        check_pos(&parent.children[0], 0.0, 0.0);
        check_size(&parent.children[1], 40.0, 40.0);
        check_pos(&parent.children[1], 0.0, 30.0);
        check_size(&parent.children[2], 50.0, 30.0);
        check_pos(&parent.children[2], 0.0, 70.0);
    }

    #[test]
    fn test_bottom_to_top() {
        let mut parent = Container::new("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0))
            .with_direction(Direction::BottomToTop)
            .with_children(vec![
                Container::new("B1", Sizing::Fixed(30.0), Sizing::Fixed(30.0)),
                Container::new("B2", Sizing::Fixed(40.0), Sizing::Fixed(40.0)),
                Container::new("B3", Sizing::Fixed(50.0), Sizing::Fill),
            ]);

        calculate_layout(&mut parent, [100.0, 100.0]);

        check_size(&parent.children[0], 30.0, 30.0);
        check_pos(&parent.children[0], 0.0, 70.0);
        check_size(&parent.children[1], 40.0, 40.0);
        check_pos(&parent.children[1], 0.0, 30.0);
        check_size(&parent.children[2], 50.0, 30.0);
        check_pos(&parent.children[2], 0.0, 0.0);
    }

    #[test]
    fn test_fill_cases() {
        let mut parent = Container::new("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0))
            .with_direction(Direction::LeftToRight)
            .with_children(vec![
                Container::new("R1", Sizing::Fixed(50.0), Sizing::Fixed(50.0)),
                Container::new("H1", Sizing::Fill, Sizing::Fill).with_children(vec![Container::new(
                    "R2",
                    Sizing::Percentage(50.0),
                    Sizing::Percentage(50.0),
                )]),
                Container::new("H2", Sizing::Fill, Sizing::Fill),
            ]);

        calculate_layout(&mut parent, [100.0, 100.0]);

        check_size(&parent, 100.0, 100.0);
        check_size(&parent.children[0], 50.0, 50.0);
        check_size(&parent.children[1], 25.0, 100.0);
        check_size(&parent.children[1].children[0], 12.5, 50.0);
        check_size(&parent.children[2], 25.0, 100.0);
    }

    #[test]
    fn test_nested_overflow() {
        let mut parent = Container::new("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0))
            .with_direction(Direction::LeftToRight)
            .with_children(vec![
                Container::new("H1", Sizing::Percentage(50.0), Sizing::Percentage(50.0)).with_children(vec![
                    Container::new("R", Sizing::Percentage(50.0), Sizing::Percentage(50.0)),
                ]),
                Container::new("H2", Sizing::Percentage(100.0), Sizing::Percentage(100.0)),
            ]);

        calculate_layout(&mut parent, [100.0, 100.0]);

        check_size(&parent, 100.0, 100.0);
        check_size(&parent.children[0], 50.0, 50.0);
        check_size(&parent.children[0].children[0], 25.0, 25.0);
        check_size(&parent.children[1], 100.0, 100.0);
    }

    #[test]
    fn test_nested_percentage() {
        let mut parent =
            Container::new("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0)).with_children(vec![Container::new(
                "H1",
                Sizing::Percentage(50.0),
                Sizing::Percentage(50.0),
            )
            .with_children(vec![Container::new(
                "H2",
                Sizing::Percentage(50.0),
                Sizing::Percentage(50.0),
            )])]);

        calculate_layout(&mut parent, [100.0, 100.0]);

        check_size(&parent, 100.0, 100.0);
        check_size(&parent.children[0], 50.0, 50.0);
        check_size(&parent.children[0].children[0], 25.0, 25.0);
    }

    #[test]
    fn test_fit_with_fill() {
        let mut parent =
            Container::new("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0)).with_children(vec![Container::new(
                "G1",
                Sizing::Fill,
                Sizing::Fill,
            )
            .with_children(vec![Container::new("G2", Sizing::Fit, Sizing::Fit).with_children(
                vec![Container::new("R", Sizing::Fixed(10.0), Sizing::Fixed(10.0))],
            )])]);

        calculate_layout(&mut parent, [100.0, 100.0]);

        check_size(&parent, 100.0, 100.0);
        check_size(&parent.children[0], 100.0, 100.0);
        check_size(&parent.children[0].children[0], 10.0, 10.0);
        check_size(&parent.children[0].children[0].children[0], 10.0, 10.0);
    }

    #[test]
    #[should_panic(expected = "Fit containers cannot have Percentage or Fill children")]
    fn test_invalid_fit_with_percentage() {
        let mut container = Container::new("H1", Sizing::Fit, Sizing::Fit).with_children(vec![Container::new(
            "R",
            Sizing::Percentage(50.0),
            Sizing::Fixed(10.0),
        )]);

        calculate_layout(&mut container, [100.0, 100.0]);
    }

    #[test]
    #[should_panic(expected = "Fit containers cannot have Percentage or Fill children")]
    fn test_invalid_fit_with_fill() {
        let mut container = Container::new("H1", Sizing::Fit, Sizing::Fit).with_children(vec![Container::new(
            "R",
            Sizing::Fill,
            Sizing::Fixed(10.0),
        )]);

        calculate_layout(&mut container, [100.0, 100.0]);
    }
}

fn main() {
    println!("Run tests with: cargo test");
}
