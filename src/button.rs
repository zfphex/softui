use crate::*;
use mini::info;

//Old version for testing.
pub fn btn(ctx: &Context) -> Button {
    Button {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::WHITE,
        ctx,
    }
}

pub fn button() -> Button<'static> {
    Button {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::WHITE,
        ctx: ctx(),
    }
}

#[derive(Clone)]
pub struct Button<'a> {
    pub area: Rect,
    pub ctx: &'a Context,
    bg: Color,
}

impl<'a> Button<'a> {}

impl<'a> Widget for Button<'a> {
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
}

impl<'a> Style for Button<'a> {
    fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}

impl<'a> Layout for Button<'a> {}
