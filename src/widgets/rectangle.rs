use crate::*;

pub const fn rect() -> Rectangle {
    Rectangle {
        size: Size {
            x: Unit::Pixel(0),
            y: Unit::Pixel(0),
            width: Unit::Pixel(10),
            height: Unit::Pixel(10),
            widgets_left: None,
        },
        bg: white(),
        radius: 0,
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub size: Size,
    pub radius: usize,
    bg: Color,
}

impl Rectangle {
    pub const fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }
}

impl<'a> Widget<'a> for Rectangle {
    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }

    fn size(&mut self, _: Rect) {
        //TODO: Should this be the default blanket implementation.
    }

    fn size_new(&self, _: Size) -> Size {
        self.size.clone()
    }

    fn position(&mut self, area: Rect) {
        self.size = area.into();
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        let bg = style.unwrap_or(Style::new()).background_color.unwrap_or(white());
        commands.push(Command {
            area: self.size.clone().into_rect(),
            primative: Primative::Ellipse(self.radius, bg),
        });
    }
}
