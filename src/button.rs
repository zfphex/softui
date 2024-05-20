use crate::*;

pub fn button(ctx: &Canvas) -> Button {
    Button {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::White,
        ctx,
        skip_draw: false,
    }
}

//TODO: missing `draw()` and `no_draw()` functions
pub struct Button<'a> {
    pub area: Rect,
    pub ctx: &'a Canvas,
    bg: Color,

    skip_draw: bool,
}

impl<'a> Draw for Button<'a> {
    fn draw(&self) {
        unsafe {
            COMMAND_QUEUE.push(Command::Rectangle(
                self.area.left as usize,
                self.area.top as usize,
                self.area.right as usize,
                self.area.bottom as usize,
                self.bg.into(),
            ));
        }
    }

    fn no_draw(&mut self) {
        self.skip_draw = true;
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

impl<'a> Layout for Button<'a> {
    fn centered(mut self) -> Self {
        let area = self.ctx.area.clone();
        let v = area.width() / 2;
        let h = area.height() / 2;

        let button_width = 10;
        let button_height = 10;

        let x = v - button_width / 2;
        let y = h - button_height / 2;
        let area = Rect::new(x, y, button_width, button_height);

        self.area = area;
        self
    }
}

impl<'a> Input for Button<'a> {
    fn clicked(&self) -> bool {
        self.ctx.left_mouse.released
            && self.ctx.mouse_pos.intersects(self.area.clone())
            && self
                .ctx
                .left_mouse
                .inital_position
                .intersects(self.area.clone())
    }

    fn up(&self, button: MouseButton) -> bool {
        if !self.ctx.mouse_pos.intersects(self.area.clone()) {
            return false;
        }

        match button {
            MouseButton::Left => self.ctx.left_mouse.released == true,
            MouseButton::Right => self.ctx.right_mouse.released == true,
            MouseButton::Middle => self.ctx.middle_mouse.released == true,
            MouseButton::Back => self.ctx.mouse_4.released == true,
            MouseButton::Forward => self.ctx.mouse_5.released == true,
        }
    }

    fn down(&self, button: MouseButton) -> bool {
        if !self.ctx.mouse_pos.intersects(self.area.clone()) {
            return false;
        }

        match button {
            MouseButton::Left => self.ctx.left_mouse.pressed == true,
            MouseButton::Right => self.ctx.right_mouse.pressed == true,
            MouseButton::Middle => self.ctx.middle_mouse.pressed == true,
            MouseButton::Back => self.ctx.mouse_4.pressed == true,
            MouseButton::Forward => self.ctx.mouse_5.pressed == true,
        }
    }
}
