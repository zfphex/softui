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
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub layout: TaffyLayout,
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
    fn w(mut self, w: impl IntoDimension) -> Self {
        self.layout.size.width = w.into_dimension();
        self
    }

    fn h(mut self, h: impl IntoDimension) -> Self {
        self.layout.size.height = h.into_dimension();
        self
    }

    fn wh(mut self, wh: impl IntoDimension) -> Self {
        let wh = wh.into_dimension();
        self.layout.size.width = wh;
        self.layout.size.height = wh;
        self
    }
}

impl<'a> Widget<'a> for Rectangle {
    fn layout(&self) -> taffy::Style {
        self.layout.clone()
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        let style = style.unwrap_or(Style::new());
        commands.push(Command {
            area,
            primative: Primative::Ellipse(
                self.radius,
                style.border_color,
                style.background_color.unwrap_or(white()),
            ),
        });
    }
}
