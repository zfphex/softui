use std::ops::Range;

use crate::*;

pub fn list<'a, 'b>(widgets: &'a [Box<dyn Widget<'b> + 'b>]) -> List<'a, 'b> {
    List {
        layout: fitstyle(),
        widgets,
        range: None,
    }
}

#[derive(Debug)]
pub struct List<'a, 'b> {
    pub layout: TaffyLayout,
    pub widgets: &'a [Box<dyn Widget<'b> + 'b>],
    pub range: Option<Range<usize>>,
}

impl<'a, 'b> Widget<'a> for List<'a, 'b> {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        //TODO: Only draw inside of window.
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }
}

//TODO: Implement the taffy traits and let list have it's own layout.
//While calculating the layout, if the size exceeds the available space, skip the rest of the widgets.

//Could also use a different NodeKind for List.
