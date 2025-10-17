use crate::*;

pub fn list() -> List {
    List { layout: fitstyle() }
}

#[derive(Debug)]
pub struct List {
    pub layout: TaffyLayout,
}

impl<'a> Widget<'a> for List {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }
}
