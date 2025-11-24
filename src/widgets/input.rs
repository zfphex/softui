use crate::*;

pub fn input(label: &str) -> Input {
    Input {}
}

#[derive(Debug)]
pub struct Input {}

impl<'a> Widget<'a> for Input {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        todo!()
    }

    fn layout(&self) -> TaffyLayout {
        todo!()
    }

    // fn on_click<F>(self, button: MouseButton, func: F) -> GenericWidget<'a, Self>
    // where
    //     Self: Sized,
    //     F: FnMut(&mut Self) + 'a,
    // {
    //     let generic = GenericWidget::new(self).on_click(button, func);
    //     self.handlers.push((button, MouseAction::Clicked, Box::new(func)));
    //     self
    // }

    fn try_click(&mut self, ctx: &mut Context, area: Rect) {}
}
