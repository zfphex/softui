use softui::*;

pub trait Position: Sized + AnyWidget {
    fn x(mut self, x: usize) -> Self {
        let area = self.area_mut();
        area.x = x;
        self
    }
}

pub trait AnyWidget {
    fn area(&self) -> Rect;
    fn area_mut(&mut self) -> &mut Rect;
    fn behaviour(&self) {}
    fn primative(&self) {}
}

struct V {
    area: Rect,
}

impl Position for V {}

impl AnyWidget for V {
    fn area(&self) -> Rect {
        self.area
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
}

fn main() {
    let widget = V { area: Rect::default() }.x(10);
    let dyn_widget = &widget as &dyn AnyWidget;
    dbg!(dyn_widget.area());
}
