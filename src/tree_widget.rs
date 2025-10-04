use crate::{tree::*, tree_simplier::*, MouseAction, MouseButton, Style};

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
    #[derive(Clone)]
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

pub trait Widget<'a> {
    fn desired_size(&self) -> [Unit; 2];

    fn w(self, w: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).w(w)
    }

    fn h(self, h: impl Into<Unit>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).h(h)
    }
}

//This is basically just a node...
//Contains all of the important information for a widget.
//Allows the user to only implement desired_size and leave the rest to us.
pub struct GenericWidget<'a, W: Widget<'a>> {
    pub widget: W,
    pub click_handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
    pub desired_size: [Unit; 2],
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub min_size: [Option<Unit>; 2],
    pub max_size: [Option<Unit>; 2],
    pub padding: f32,
    pub margin: f32,
    pub style: Option<Style>,
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn new(widget: W) -> Self {
        GenericWidget {
            desired_size: widget.desired_size(),
            widget,
            click_handlers: Vec::new(),
            size: [0.0; 2],
            pos: [0.0; 2],
            min_size: [None; 2],
            max_size: [None; 2],
            padding: 0.0,
            margin: 0.0,
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
            padding: Amount {
                top: self.padding,
                bottom: self.padding,
                left: self.padding,
                right: self.padding,
            },
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

    pub fn h(mut self, h: impl Into<Unit>) -> Self {
        self.desired_size[1] = h.into();
        self
    }

    pub fn padding(mut self, padding: impl IntoF32) -> Self {
        self.padding = padding.into_f32();
        self
    }

    pub fn margin(mut self, margin: impl IntoF32) -> Self {
        self.margin = margin.into_f32();
        self
    }
}
