use crate::*;

pub struct RectangleNew {
    pub area: Rect,
    pub radius: usize,
    pub on_clicked: Option<Box<dyn FnMut(&mut RectangleNew)>>,
    bg: Color,
}

impl std::fmt::Debug for RectangleNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RectangleNew")
            .field("area", &self.area)
            .field("radius", &self.radius)
            // .field("on_clicked", &self.on_clicked)
            .field("bg", &self.bg)
            .finish()
    }
}

impl RectangleNew {
    pub fn new() -> Self {
        RectangleNew {
            area: Rect::new(0, 0, 10, 10),
            bg: Color::WHITE,
            on_clicked: None,
            radius: 0,
        }
    }
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }
    // pub fn on_clicked(mut self, on_clicked: impl FnMut(&mut RectangleNew)) -> Self {
    //     self.on_clicked = Some(Box::new(on_clicked));
    //     self
    // }
    // pub fn on_clicked<F: FnMut(&mut Self) + 'a>(mut self, on_clicked: dyn FnMut(&mut RectangleNew)) -> Self {
    //     self.on_clicked = Some(Box::new(on_clicked));
    //     self
    // }
    fn temp_on_clicked(&mut self, button: MouseButton) {
        let ctx = ctx();

        if Self::is_container() {
            todo!();
            // self.adjust_position(0, 0);
        }

        let area = self.area().unwrap().clone();

        if !ctx.mouse_pos.intersects(area) {
            return;
        }

        let clicked = match button {
            MouseButton::Left => {
                ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
            }
            MouseButton::Right => {
                ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
            }
            MouseButton::Middle => {
                ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
            }
            MouseButton::Back => {
                ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area)
            }
            MouseButton::Forward => {
                ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area)
            }
        };

        if clicked {
            // function(&mut self);
            if let Some(mut function) = self.on_clicked.take() {
                function(self);
            }
        }
    }
}

impl Widget for RectangleNew {
    fn draw(&self) -> Option<Command> {
        // self.temp_on_clicked(Left);
        // if let Some(click) = &mut self.on_clicked {
        //     click();
        // }

        // unsafe {
        //     COMMAND_QUEUE.push(Command::Ellipse(
        //         self.area.x as usize,
        //         self.area.y as usize,
        //         self.area.width as usize,
        //         self.area.height as usize,
        //         self.radius,
        //         self.bg,
        //     ));
        // }
        todo!();
    }

    #[inline]
    fn area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

impl Style for RectangleNew {
    fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}
