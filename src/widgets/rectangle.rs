use crate::*;

pub const fn rect() -> Rectangle {
    Rectangle {
        area: Rect::new(0, 0, 10, 10),
        bg: white(),
        radius: 0,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub area: Rect,
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
    fn size(&self) -> (usize, usize) {
        (self.area.width, self.area.height)
    }
    fn layout(&mut self, area: Rect) {
        self.area = area;
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
    fn handle_event(&mut self, _ctx: &mut Context) {}
    fn draw(&self, commands: &mut Vec<Command>) {
        commands.push(Command {
            area: self.area,
            primative: Primative::Ellipse(self.radius, self.bg),
        });
    }
}

impl StyleNew for Rectangle {
    fn set_bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}
