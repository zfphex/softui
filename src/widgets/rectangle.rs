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
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub layout: TaffyLayout,
    pub radius: usize,
}

impl Rectangle {
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }
}

impl<'a> Widget<'a> for Rectangle {
    fn layout(&self) -> taffy::Style {
        self.layout.clone()
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        let bg = style.unwrap_or(Style::new()).background_color.unwrap_or(white());
        commands.push(Command {
            area,
            primative: Primative::Ellipse(self.radius, bg),
        });
    }
}
