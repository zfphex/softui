use crate::*;

pub fn button<'a>(label: &'a str) -> Button<'a> {
    Button {
        label,
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
pub struct Button<'a> {
    pub label: &'a str,
    pub radius: usize,
    pub outline: bool,
    pub widget: WidgetData,
}

impl<'a> Button<'a> {
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }

    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    pub fn clicked(&self, ctx: &mut Context) -> bool {
        clicked(ctx, self.widget.area.get(), Left)
    }
}

impl<'a> Sizing for Button<'a> {
    fn layout_mut(&mut self) -> &mut TaffyLayout {
        &mut self.widget.layout
    }
}

impl<'a> Styling for Button<'a> {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.widget.style
    }
}

impl<'a> Widget<'a> for Button<'a> {
    fn layout(&self) -> taffy::Style {
        self.widget.layout.clone()
    }

    fn primitive(&self) -> Primative {
        let style = self.widget.style;
        Primative::Ellipse(self.radius, style.border_color, style.background_color)
    }

    fn area_cell(&'a self) -> Option<&'a std::cell::Cell<Rect>> {
        Some(&self.widget.area)
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        let style = self.widget.style;
        commands.push(Command {
            area,
            primative: Primative::Ellipse(self.radius, style.border_color, style.background_color),
        });
    }
}
