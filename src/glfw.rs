use crate::{Backend, Rect};
use glfw::{Context, GlfwReceiver, WindowEvent};

pub struct Glfw {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Glfw {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(800, 600, "Softui", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();
        Self {
            glfw,
            window,
            events,
        }
    }
}

impl Backend for Glfw {
    fn area(&self) -> Rect {
        let (width, height) = self.window.get_size();
        Rect {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    fn buffer<'a>(&self) -> &'a mut [u32] {
        todo!()
    }

    fn resize(&self) {}

    fn present(&self) {
        todo!()
    }

    fn event(&mut self) -> Option<crate::Event> {
        while !self.window.should_close() {
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                dbg!(event);
            }
        }
        return None;
    }
}
