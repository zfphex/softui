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

#[derive(Clone)]
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
    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    fn area(&self) -> Option<&Rect> {
        Some(&self.area)
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
                let percentage = p as f32 / 100.0;
                self.area.x = ((self.parent_area.width as f32 * percentage)
                    - (self.area.width as f32 / 2.0))
                    .round() as i32;
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

//TODO: Simplify this down even more.
impl<'a> Input for Button<'a> {
    fn on_clicked<F: FnMut(&Context) -> ()>(self, button: MouseButton, mut function: F) -> Self {
        if clicked(self.ctx, &self, button) {
            function(self.ctx);
        }
        self
    }

    #[inline]
    fn clicked(&self, button: MouseButton) -> bool {
        clicked(self.ctx, self, button)
    }

    #[inline]
    fn up(&self, button: MouseButton) -> bool {
        up(self.ctx, self, button)
    }

    #[inline]
    fn down(&self, button: MouseButton) -> bool {
        down(self.ctx, self, button)
    }
}
