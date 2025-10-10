use crate::{IntoF32, MouseAction, MouseButton, Style, tree::*, tree_simplier::*};

pub fn rect() -> Rectangle {
    Rectangle {
        size: Size {
            pos: [0.0; 2],
            dimensions: [Unit::Fill, Unit::Fill],
        },
        radius: 0,
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub size: Size,
    pub radius: usize,
}

impl<'a> Widget<'a> for Rectangle {
    fn desired_size(&self) -> [Unit; 2] {
        self.size.dimensions
    }
}

// impl IntoNode for Rectangle {
//     fn into_node(self) -> Node {
//         Node {
//             pos: self.size.pos,
//             desired_size: self.size.dimensions,
//             ..Default::default()
//         }
//     }
// }

#[cfg(feature = "image")]
pub mod image_ref {
    use super::*;

    pub fn image_ref<'a>(image: &'a crate::Image) -> ImageRef<'a> {
        ImageRef {
            format: image.format,
            size: [
                Unit::Fixed(image.area.width as f32),
                Unit::Fixed(image.area.height as f32),
            ],
            bitmap: &image.bitmap,
        }
    }

    //Test widget.
    #[derive(Clone, Debug)]
    pub struct ImageRef<'a> {
        pub format: crate::widgets::image::ImageFormat,
        pub size: [Unit; 2],
        pub bitmap: &'a [u8],
    }

    impl IntoNode for ImageRef<'_> {
        fn into_node(self) -> Node {
            Node {
                desired_size: self.size,
                ..Default::default()
            }
        }
    }

    impl<'a> Widget<'a> for ImageRef<'a> {
        fn desired_size(&self) -> [Unit; 2] {
            self.size
        }
    }
}

pub trait Widget<'a>: std::fmt::Debug {
    fn desired_size(&self) -> [Unit; 2];

    fn w(self, w: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).w(w)
    }
    fn max_w(self, w: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).max_w(w)
    }
    fn min_w(self, w: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).min_w(w)
    }
    fn h(self, h: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).h(h)
    }
    fn max_h(self, h: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).max_h(h)
    }
    fn min_h(self, h: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).min_h(h)
    }
    fn wh(self, wh: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).wh(wh)
    }
    fn wfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).wfill()
    }
    fn hfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).hfill()
    }
    fn whfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).whfill()
    }
    #[inline(always)]
    fn on_click<F>(self, button: MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_click(button, handler)
    }
    #[inline(always)]
    fn on_press<F>(self, button: MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_press(button, handler)
    }
    #[inline(always)]
    fn on_release<F>(self, button: MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_release(button, handler)
    }
    fn try_click(&mut self) {}
}

//This is basically just a node...
//Contains all of the important information for a widget.
//Allows the user to only implement desired_size and leave the rest to us.
pub struct GenericWidget<'a, W: Widget<'a>> {
    pub widget: W,
    pub handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
    pub desired_size: [Unit; 2],
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub min_size: [Option<Unit>; 2],
    pub max_size: [Option<Unit>; 2],
    pub padding: Amount,
    pub margin: Amount,
    pub style: Option<Style>,
}

impl<'a, W: Widget<'a>> std::fmt::Debug for GenericWidget<'a, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericWidget")
            .field("widget", &self.widget)
            .field(
                "click_handlers",
                &self
                    .handlers
                    .iter()
                    .map(|(button, action, _)| format!("{:?} {:?}", button, action))
                    .collect::<Vec<_>>(),
            )
            .field("desired_size", &self.desired_size)
            .field("pos", &self.pos)
            .field("size", &self.size)
            .field("min_size", &self.min_size)
            .field("max_size", &self.max_size)
            .field("padding", &self.padding)
            .field("margin", &self.margin)
            .field("style", &self.style)
            .finish()
    }
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn new(widget: W) -> Self {
        GenericWidget {
            desired_size: widget.desired_size(),
            widget,
            handlers: Vec::new(),
            size: [0.0; 2],
            pos: [0.0; 2],
            min_size: [None; 2],
            max_size: [None; 2],
            padding: Amount::splat(0.0),
            margin: Amount::splat(0.0),
            style: None,
        }
    }
}

impl<'a, W: Widget<'a>> IntoNode for GenericWidget<'a, W> {
    fn into_node(self) -> Node {
        Node {
            desired_size: self.desired_size,
            size: self.size,
            pos: self.pos,
            padding: self.padding,
            margin: self.margin,
            min_size: self.min_size,
            max_size: self.max_size,
            style: self.style,
            ..Default::default()
        }
    }
}

impl<'a, W: Widget<'a>> Widget<'a> for GenericWidget<'a, W> {
    fn desired_size(&self) -> [Unit; 2] {
        self.widget.desired_size()
    }
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn w(mut self, w: impl Into<Unit>) -> Self {
        self.desired_size[0] = w.into();
        self
    }

    fn max_w(mut self, w: impl Into<Unit>) -> Self
    where
        Self: Sized,
    {
        self.max_size[0] = Some(w.into());
        self
    }

    fn min_w(mut self, w: impl Into<Unit>) -> Self
    where
        Self: Sized,
    {
        self.min_size[0] = Some(w.into());
        self
    }

    pub fn h(mut self, h: impl Into<Unit>) -> Self {
        self.desired_size[1] = h.into();
        self
    }

    fn max_h(mut self, h: impl Into<Unit>) -> Self
    where
        Self: Sized,
    {
        self.max_size[1] = Some(h.into());
        self
    }

    fn min_h(mut self, h: impl Into<Unit>) -> Self
    where
        Self: Sized,
    {
        self.min_size[1] = Some(h.into());
        self
    }

    pub fn wh(mut self, wh: impl Into<Unit>) -> Self {
        let unit = wh.into();
        self.desired_size[0] = unit;
        self.desired_size[1] = unit;
        self
    }

    pub fn wfill(mut self) -> Self {
        self.desired_size[0] = Unit::Fill;
        self
    }

    pub fn hfill(mut self) -> Self {
        self.desired_size[1] = Unit::Fill;
        self
    }

    fn whfill(mut self) -> Self
    where
        Self: Sized,
    {
        self.desired_size[0] = Unit::Fill;
        self.desired_size[1] = Unit::Fill;
        self
    }

    pub fn padding(mut self, padding: impl IntoF32) -> Self {
        self.padding = Amount::splat(padding.into_f32());
        self
    }

    pub fn pl(mut self, left: impl IntoF32) -> Self {
        self.padding.left = left.into_f32();
        self
    }

    pub fn pr(mut self, right: impl IntoF32) -> Self {
        self.padding.right = right.into_f32();
        self
    }

    pub fn pt(mut self, top: impl IntoF32) -> Self {
        self.padding.top = top.into_f32();
        self
    }

    pub fn pb(mut self, bottom: impl IntoF32) -> Self {
        self.padding.bottom = bottom.into_f32();
        self
    }

    pub fn on_click(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Clicked, Box::new(handler)));
        self
    }

    pub fn on_press(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Pressed, Box::new(handler)));
        self
    }

    pub fn on_release(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Released, Box::new(handler)));
        self
    }
    pub fn try_click(&mut self) {
        for handler in &mut self.handlers {}
    }
}
