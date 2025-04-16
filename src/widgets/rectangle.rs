use crate::*;

//Old version.
// pub fn rct(ctx: &Context) -> Rectangle {
//     Rectangle {
//         area: Rect::new(0, 0, 10, 10),
//         bg: Color::WHITE,
//         radius: 0,
//         ctx,
//     }
// }

pub fn rect() -> Rectangle {
    Rectangle {
        area: Rect::new(0, 0, 10, 10),
        bg: white(),
        radius: 0,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub area: Rect,
    pub radius: usize,
    bg: Color,
}

impl Rectangle {
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }
}

impl Widget for Rectangle {
    fn primative(&self) -> Primative {
        Primative::Ellipse(self.radius, self.bg)
    }

    #[inline]
    fn area(&self) -> Rect {
        self.area
    }

    #[inline]
    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

impl Style for Rectangle {
    fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}
