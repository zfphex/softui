use softui::*;

pub trait WidgetNew: Sized + AnyWidget {
    fn x(mut self, x: usize) -> Self {
        let area = self.area_mut();
        area.x = x;
        self
    }

    fn on_click(mut self, button: MouseButton, function: fn(&mut dyn AnyWidget)) -> Self {
        if let Some(behaviour) = self.behaviour_mut() {
            behaviour.push(AnyClick {
                button,
                action: MouseAction::Clicked,
                function,
            });
        }
        self
    }
}

pub struct AnyClick {
    pub button: MouseButton,
    pub action: MouseAction,
    pub function: fn(&mut dyn AnyWidget),
}

pub trait AnyWidget {
    fn area(&self) -> Rect;
    fn area_mut(&mut self) -> &mut Rect;
    fn primative(&self) {}
    fn behaviour(&mut self) -> Option<&[AnyClick]> {
        None
    }
    fn behaviour_mut(&mut self) -> Option<&mut Vec<AnyClick>> {
        None
    }
}

struct V {
    area: Rect,
    behaviour: Vec<AnyClick>,
}

impl WidgetNew for V {}

impl AnyWidget for V {
    fn area(&self) -> Rect {
        self.area
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
    fn behaviour(&mut self) -> Option<&[AnyClick]> {
        Some(&self.behaviour)
    }
    fn behaviour_mut(&mut self) -> Option<&mut Vec<AnyClick>> {
        Some(&mut self.behaviour)
    }
}

fn widgets(widgets: &mut [&mut dyn AnyWidget]) {
    for widget in widgets {
        if let Some(behaviours) = widget.behaviour() {
            for b in behaviours {
                // (b.function)(*widget);
            }
        }
    }
}

fn main() {
    let mut widget = V {
        area: Rect::default(),
        behaviour: Vec::new(),
    }
    .x(10)
    .on_click(Left, |s| println!("Clicked left mouse."));

    let dyn_widget = &mut widget as &mut dyn AnyWidget;
    widgets(&mut [dyn_widget]);
}
