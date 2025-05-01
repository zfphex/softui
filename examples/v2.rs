use softui::*;

pub trait Position: Sized + AnyWidget {
    fn x(self, x: usize) -> Self;
}

pub trait AnyWidget {
    fn area(&self) -> Rect;
    fn behaviour(&self) {}
    fn primative(&self) {}
}

struct V {
    x: usize,
}

impl Position for V {
    fn x(mut self, x: usize) -> Self {
        self.x = x;
        self
    }
}

impl AnyWidget for V {
    fn area(&self) -> Rect {
        Rect::new(self.x, 0, 0, 0)
    }
}

fn main() {
    let widget = V { x: 10 };
    let dyn_widget = &widget as &dyn AnyWidget;
    dbg!(dyn_widget.area());
}
