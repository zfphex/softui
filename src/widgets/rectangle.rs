use crate::*;

pub fn rect() -> Rectangle {
    Rectangle {
        layout: TaffyLayout {
            size: taffy::Size {
                width: taffy::style_helpers::length(20.0),
                height: taffy::style_helpers::length(20.0),
            },
            ..Default::default()
        },
        radius: 0,
        outline: false,
        style: Style::new(),
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub layout: TaffyLayout,
    pub style: Style,
    pub radius: usize,
    pub outline: bool,
}

impl Rectangle {
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }

    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }
}

impl Sizing for Rectangle {
    fn layout_mut(&mut self) -> &mut TaffyLayout {
        &mut self.layout
    }
}

impl Styling for Rectangle {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<'a> Widget<'a> for Rectangle {
    fn layout(&self) -> taffy::Style {
        self.layout.clone()
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        commands.push(Command {
            area,
            primative: Primative::Ellipse(
                self.radius,
                self.style.border_color,
                self.style.background_color.unwrap_or(white()),
            ),
        });
    }
}
