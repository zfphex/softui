use crate::*;

//Old version for testing.
pub fn rct(ctx: &Context) -> Rectangle {
    Rectangle {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::WHITE,
        radius: 0,
        ctx,
    }
}

pub fn rect() -> Rectangle<'static> {
    Rectangle {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::WHITE,
        radius: 0,
        ctx: ctx(),
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle<'a> {
    pub area: Rect,
    pub ctx: &'a Context,
    pub radius: usize,
    bg: Color,
}

impl<'a> Rectangle<'a> {
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }
}

impl<'a> Widget for Rectangle<'a> {
    fn draw_command(&self) -> Option<Command> {
        Some(Command::Ellipse(
            self.area.x as usize,
            self.area.y as usize,
            self.area.width as usize,
            self.area.height as usize,
            self.radius,
            self.bg,
        ))
    }

    #[inline]
    fn area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

impl<'a> Style for Rectangle<'a> {
    fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}
