use crate::*;

pub const fn rect() -> Rectangle {
    Rectangle {
        area: UnitRect {
            x: Unit::Pixel(0),
            y: Unit::Pixel(0),
            width: Unit::Pixel(10),
            height: Unit::Pixel(10),
        },
        bg: white(),
        radius: 0,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub area: UnitRect,
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
    fn area_mut(&mut self) -> &mut UnitRect {
        &mut self.area
    }

    fn size(&self, parent: Rect) -> Size {
        Size {
            width: self.area.width,
            height: self.area.height,
            remaining_widgets: None,
        }
    }

    fn position(&mut self, current_size: Size, parent: Rect) {
        // self.area = parent;
        self.area = parent.into();
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        let bg = style.unwrap_or(Style::new()).background_color.unwrap_or(white());
        commands.push(Command {
            area: self.area.into_rect(),
            primative: Primative::Ellipse(self.radius, bg),
        });
    }
}
