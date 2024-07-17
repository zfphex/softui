use crate::*;
use mini::info;

//TODO: Really this should take any parent.
//We don't have layout widgets yet.
pub fn button(ctx: &Context) -> Button {
    Button {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::White,
        ctx,
        skip_draw: false,
    }
}

pub fn button2() -> Button<'static> {
    Button {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::White,
        ctx: ctx(),
        skip_draw: false,
    }
}

#[derive(Clone)]
pub struct Button<'a> {
    pub area: Rect,
    pub ctx: &'a Context,
    bg: Color,
    skip_draw: bool,
}

impl<'a> Button<'a> {}

impl<'a> Widget for Button<'a> {
    fn draw(&self) {
        unsafe {
            COMMAND_QUEUE.push(Command::Rectangle(
                self.area.x as usize,
                self.area.y as usize,
                self.area.width as usize,
                self.area.height as usize,
                self.bg.into(),
            ));
        }
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    fn area(&self) -> Option<&Rect> {
        Some(&self.area)
    }
}

impl<'a> Drop for Button<'a> {
    fn drop(&mut self) {
        if !self.skip_draw {
            self.draw()
        }
    }
}

impl<'a> Style for Button<'a> {
    fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}

impl<'a> Layout for Button<'a> {}

//TODO: Simplify this down even more.
impl<'a> Input for Button<'a> {
    fn on_clicked<F: FnMut(&Context) -> ()>(self, button: Mouse, mut function: F) -> Self {
        if clicked(self.ctx, &self, button) {
            function(self.ctx);
        }
        self
    }

    #[inline]
    fn clicked(&self, button: Mouse) -> bool {
        clicked(self.ctx, self, button)
    }

    #[inline]
    fn up(&self, button: Mouse) -> bool {
        up(self.ctx, self, button)
    }

    #[inline]
    fn down(&self, button: Mouse) -> bool {
        down(self.ctx, self, button)
    }
}
