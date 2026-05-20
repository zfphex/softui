use crate::*;

pub fn rect() -> Rectangle {
    Rectangle {
        radius: 0,
        outline: false,
        widget: WidgetData {
            area: std::cell::Cell::new(Rect::default()),
            layout: TaffyLayout {
                size: taffy::Size {
                    width: taffy::style_helpers::length(20.0_f32),
                    height: taffy::style_helpers::length(20.0_f32),
                },
                ..Default::default()
            },
            style: Style::new().bg(white()),
        },
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub radius: usize,
    pub outline: bool,
    pub widget: WidgetData,
}

impl Rectangle {
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }

    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }
}

impl Sizing for Rectangle {
    fn layout_mut(&mut self) -> &mut TaffyLayout {
        &mut self.widget.layout
    }
}

impl Styling for Rectangle {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.widget.style
    }
}

impl<'a> Widget<'a> for Rectangle {
    fn layout(&self) -> taffy::Style {
        self.widget.layout.clone()
    }

    fn area_cell(&'a self) -> Option<&'a std::cell::Cell<Rect>> {

        // let width = self.widget.layout.size.width.into_raw();
        // let height = self.widget.layout.size.height;
        // self.widget.area.set(taffy::Size { width, height })
        Some(&self.widget.area)
    }

    fn primitive(&self) -> Option<Primative> {
        Some(Primative::Ellipse(
            self.radius,
            self.widget.style.foreground_color,
            self.widget.style.background_color,
        ))
    }
}
