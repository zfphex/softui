use crate::*;
use mini::info;

pub fn rect_new<F: FnMut() -> ()>() -> RectangleNew<F> {
    RectangleNew {
        area: Rect::new(0, 0, 10, 10),
        bg: Color::WHITE,
        on_clicked: None,
        radius: 0,
    }
}

#[derive(Clone)]
pub struct RectangleNew<F: FnMut() -> ()> {
    pub area: Rect,
    pub radius: usize,
    pub on_clicked: Option<F>,
    bg: Color,
}

impl<F: FnMut() -> ()> RectangleNew<F> {
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }
    pub fn on_clicked(mut self, on_clicked: F) -> Self {
        self.on_clicked = Some(on_clicked);
        self
    }
    fn temp_on_clicked(&mut self, button: Mouse) {
        let ctx = ctx();

        if Self::is_container() {
            self.adjust_position(0, 0);
        }

        let area = self.area().unwrap();

        if !ctx.mouse_pos.intersects(area) {
            return;
        }

        let clicked = match button {
            Mouse::Left => {
                ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
            }
            Mouse::Right => {
                ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
            }
            Mouse::Middle => {
                ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
            }
            Mouse::Back => ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area),
            Mouse::Forward => ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area),
        };

        if clicked {
            // function(&mut self);
            if let Some(function) = &mut self.on_clicked {
                function();
            }
        }
    }
}

impl<F: FnMut() -> ()> Widget for RectangleNew<F> {
    fn draw(&mut self) {
        self.temp_on_clicked(Left);
        // if let Some(click) = &mut self.on_clicked {
        //     click();
        // }

        unsafe {
            COMMAND_QUEUE.push(Command::Ellipse(
                self.area.x as usize,
                self.area.y as usize,
                self.area.width as usize,
                self.area.height as usize,
                self.radius,
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

impl<F: FnMut() -> ()> Style for RectangleNew<F> {
    fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}

impl<F: FnMut() -> ()> Layout for RectangleNew<F> {
    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}
