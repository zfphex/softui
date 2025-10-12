use crate::*;

pub fn rect() -> Rectangle {
    Rectangle {
        style: taffy::Style {
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
    pub style: taffy::Style,
    pub radius: usize,
}

impl<'a> Widget<'a> for Rectangle {
    fn style(&self) -> taffy::Style {
        self.style.clone()
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        //  let bg = style.unwrap_or(TaffyStyle::new()).background_color.unwrap_or(white());
        commands.push(Command {
            area,
            primative: Primative::Ellipse(self.radius, white()),
        });
    }
}
