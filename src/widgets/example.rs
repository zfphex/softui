use crate::*;

pub fn example(label: &str) -> Example {
    Example {}
}

#[derive(Debug)]
pub struct Example {}

impl<'a> Widget<'a> for Example {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        todo!()
    }

    fn layout(&self) -> TaffyLayout {
        todo!()
    }
}
