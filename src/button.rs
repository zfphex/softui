use crate::*;
use mini::info;

//TODO: Really this should take any parent.
//We don't have layout widgets yet.
pub fn button(ctx: &Context) -> Button {
    Button {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::White,
        parent_area: &ctx.area,
        ctx,
        skip_draw: false,
    }
}

pub struct Button<'a> {
    pub area: Rect,
    pub ctx: &'a Context,
    //Not sure about this yet.
    pub parent_area: &'a Rect,

    bg: Color,
    skip_draw: bool,
}

impl<'a> Button<'a> {}

impl<'a> View for Button<'a> {
    fn area(&mut self) -> &mut Rect {
        &mut self.area
    }
}

impl<'a> Draw for Button<'a> {
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
        let parent_area = self.parent_area.clone();
        let x = (parent_area.width as f32 / 2.0) - (self.area.width as f32 / 2.0);
        let y = (parent_area.height as f32 / 2.0) - (self.area.height as f32 / 2.0);

        self.area = Rect::new(
            x.round() as i32,
            y.round() as i32,
            self.area.width,
            self.area.height,
        );

        self
    }
    //TODO: Layout should be based on the parent.
    //It don't have the mechanisms in place to handle this.
    //I think each widget should probably hold a Parent<'a>
    //Current we use the canvas which is kind of like the body.
    //But it handles input and whatnot aswell.
    //Hmmmm

    fn right<U: Into<Unit>>(mut self, length: U) -> Self {
        match length.into() {
            Unit::Px(px) => {
                // self.area.right = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }

    fn bottom<U: Into<Unit>>(mut self, length: U) -> Self {
        match length.into() {
            Unit::Px(px) => {
                // self.area.bottom -= px as i32;
                todo!()
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }

    fn x<U: Into<Unit>>(mut self, x: U) -> Self {
        match x.into() {
            Unit::Px(px) => {
                self.area.x = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(p) => {
                let px = self.parent_area.width as f32 * (p as f32 / 100.0);
                let half_width = self.area.width as f32 / 2.0;
                self.area.x = (px - half_width) as i32;
            }
        }
        self
    }

    fn y<U: Into<Unit>>(mut self, y: U) -> Self {
        match y.into() {
            Unit::Px(px) => {
                self.area.y = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }

    fn width<U: Into<Unit>>(mut self, length: U) -> Self {
        match length.into() {
            Unit::Px(px) => {
                self.area.width = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }

    fn height<U: Into<Unit>>(mut self, length: U) -> Self {
        match length.into() {
            Unit::Px(px) => {
                self.area.height = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
}

impl<'a> Input for Button<'a> {
    fn clicked(&self, button: MouseButton) -> bool {
        if !self.ctx.mouse_pos.intersects(self.area.clone()) {
            return false;
        }

        match button {
            MouseButton::Left => {
                self.ctx.left_mouse.released
                    && self
                        .ctx
                        .left_mouse
                        .inital_position
                        .intersects(self.area.clone())
            }
            MouseButton::Right => {
                self.ctx.right_mouse.released
                    && self
                        .ctx
                        .right_mouse
                        .inital_position
                        .intersects(self.area.clone())
            }
            MouseButton::Middle => {
                self.ctx.middle_mouse.released
                    && self
                        .ctx
                        .middle_mouse
                        .inital_position
                        .intersects(self.area.clone())
            }
            MouseButton::Back => {
                self.ctx.mouse_4.released
                    && self
                        .ctx
                        .mouse_4
                        .inital_position
                        .intersects(self.area.clone())
            }
            MouseButton::Forward => {
                self.ctx.mouse_5.released
                    && self
                        .ctx
                        .mouse_5
                        .inital_position
                        .intersects(self.area.clone())
            }
        }
    }

    fn up(&self, button: MouseButton) -> bool {
        if !self.ctx.mouse_pos.intersects(self.area.clone()) {
            return false;
        }

        match button {
            MouseButton::Left => self.ctx.left_mouse.released,
            MouseButton::Right => self.ctx.right_mouse.released,
            MouseButton::Middle => self.ctx.middle_mouse.released,
            MouseButton::Back => self.ctx.mouse_4.released,
            MouseButton::Forward => self.ctx.mouse_5.released,
        }
    }

    fn down(&self, button: MouseButton) -> bool {
        if !self.ctx.mouse_pos.intersects(self.area.clone()) {
            return false;
        }

        match button {
            MouseButton::Left => self.ctx.left_mouse.pressed,
            MouseButton::Right => self.ctx.right_mouse.pressed,
            MouseButton::Middle => self.ctx.middle_mouse.pressed,
            MouseButton::Back => self.ctx.mouse_4.pressed,
            MouseButton::Forward => self.ctx.mouse_5.pressed,
        }
    }
}
