use crate::*;

#[derive(Default)]
pub struct ExampleWidget {
    pub position: Rect,
    pub behaviour: Vec<Click<Self>>,
}

impl Widget for ExampleWidget {
    type Layout = Self;

    fn behaviour(&mut self) -> Option<&mut Vec<Click<Self>>> {
        Some(&mut self.behaviour)
    }

    fn primative(&self) -> Primative {
        Primative::Ellipse(0, white())
    }

    fn area(&self) -> Rect {
        self.position
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.position)
    }
}
