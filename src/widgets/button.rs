use crate::*;

pub fn button() -> Button {
    Button {
        rect: rect(),
        active: false,
        hover_color: None,
        active_color: None,
    }
}

#[derive(Clone, Debug)]
pub struct Button {
    pub rect: Rectangle,
    pub active: bool,
    pub hover_color: Option<Color>,
    pub active_color: Option<Color>,
}

impl Button {
    //Okay now what? How can I set a hover color and active color?
    //yeah my design really sucks tbh 
}

impl<'a> Widget<'a> for Button {
    fn layout(&self) -> taffy::Style {
        self.rect.layout.clone()
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        self.rect.draw(commands, area, style);
    }
}
