use crate::*;

pub const fn rect() -> Rectangle {
    Rectangle {
        area: Rect::new(0, 0, 10, 10),
        area_new: UnitRect {
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
    pub area: Rect,
    pub area_new: UnitRect,
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
    fn area_mut_new(&mut self) -> &mut UnitRect {
        &mut self.area_new
    }
    fn desired_size(&self) -> (Unit, Unit) {
        (self.area_new.width, self.area_new.height)
    }
    fn size(&self) -> (usize, usize) {
        (self.area.width, self.area.height)
    }
    fn layout(&mut self, area: Rect) {
        self.area = area;
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        let bg = style.unwrap_or(Style::new()).background_color.unwrap_or(white());
        commands.push(Command {
            area: self.area,
            primative: Primative::Ellipse(self.radius, bg),
        });
    }
}
