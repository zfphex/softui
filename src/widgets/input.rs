use crate::*;

pub fn input(label: &str) -> Input {
    Input {}
}

#[derive(Debug)]
pub struct Input {}

impl<'a> Widget<'a> for Input {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        todo!()
    }

    fn layout(&self) -> TaffyLayout {
        todo!()
    }
}
