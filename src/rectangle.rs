use crate::*;
use mini::info;

//Old version for testing.
pub fn rct(ctx: &Context) -> Rectangle {
    Rectangle {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::WHITE,
        ctx,
    }
}

pub fn rect() -> Rectangle<'static> {
    Rectangle {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::WHITE,
        ctx: ctx(),
    }
}

#[derive(Clone)]
pub struct Rectangle<'a> {
    pub area: Rect,
    pub ctx: &'a Context,
    bg: Color,
}

impl<'a> Rectangle<'a> {}

impl<'a> Widget for Rectangle<'a> {
    fn draw(&mut self) {
        unsafe {
            COMMAND_QUEUE.push(Command::Rectangle(
                self.area.x as usize,
                self.area.y as usize,
                self.area.width as usize,
                self.area.height as usize,
                self.bg,
            ));
        }
    }

    #[inline]
    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    #[inline]
    fn area(&self) -> Option<Rect> {
        Some(self.area)
    }

    fn adjust_position(&mut self, x: i32, y: i32) {
        self.area.x = x;
        self.area.y = y;
    }
}

impl<'a> Style for Rectangle<'a> {
    fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}

impl<'a> Layout for Rectangle<'a> {
    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}
