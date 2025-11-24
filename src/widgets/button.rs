use crate::*;

pub fn button(label: &str) -> Button {
    Button {}
}

#[derive(Debug)]
pub struct Button {}

impl<'a> Widget<'a> for Button {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        todo!()
    }

    fn layout(&self) -> TaffyLayout {
        todo!()
    }
}
