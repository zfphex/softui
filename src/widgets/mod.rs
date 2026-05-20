pub mod rectangle;
pub use rectangle::*;

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "svg")]
pub use svg::*;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "image")]
pub use image::*;

pub mod text;
use taffy::{AvailableSpace, BoxSizing, Dimension, Size, prelude::length};
pub use text::*;

pub use taffy::AlignItems;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub mod dwrite;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub use dwrite::*;

pub mod str;

use crate::*;

pub trait Styling: Sized {
    fn style_mut(&mut self) -> &mut Style;

    fn bg(mut self, bg: impl IntoColor) -> Self {
        self.style_mut().background_color = bg.into_color();
        self
    }

    fn fg(mut self, fg: impl IntoColor) -> Self {
        self.style_mut().foreground_color = fg.into_color();
        self
    }

    fn radius(mut self, radius: usize) -> Self {
        self.style_mut().radius = radius;
        self
    }
}

pub trait Sizing: Sized {
    #[inline]
    fn layout_mut(&mut self) -> &mut TaffyLayout;

    fn set_layout(mut self, f: impl FnOnce(&mut TaffyLayout)) -> Self {
        f(self.layout_mut());
        self
    }

    fn w(mut self, w: impl IntoDimension) -> Self {
        self.layout_mut().size.width = w.into_dimension();
        self
    }

    fn h(mut self, h: impl IntoDimension) -> Self {
        self.layout_mut().size.height = h.into_dimension();
        self
    }

    fn wh(mut self, wh: impl IntoDimension) -> Self {
        let wh = wh.into_dimension();
        let layout = self.layout_mut();
        layout.size.width = wh;
        layout.size.height = wh;
        self
    }

    fn p(mut self, padding: impl IntoF32) -> Self {
        let p = padding.into_f32();
        let layout = self.layout_mut();
        layout.padding.left = length(p);
        layout.padding.right = length(p);
        layout.padding.top = length(p);
        layout.padding.bottom = length(p);
        self
    }

    fn pl(mut self, left: impl IntoF32) -> Self {
        self.layout_mut().padding.left = length(left.into_f32());
        self
    }

    fn pr(mut self, right: impl IntoF32) -> Self {
        self.layout_mut().padding.right = length(right.into_f32());
        self
    }

    fn pt(mut self, top: impl IntoF32) -> Self {
        self.layout_mut().padding.top = length(top.into_f32());
        self
    }

    fn pb(mut self, bottom: impl IntoF32) -> Self {
        self.layout_mut().padding.bottom = length(bottom.into_f32());
        self
    }

    fn max_w(mut self, w: impl IntoDimension) -> Self {
        self.layout_mut().max_size.width = w.into_dimension();
        self
    }
    fn min_w(mut self, w: impl IntoDimension) -> Self {
        self.layout_mut().min_size.width = w.into_dimension();
        self
    }
    fn max_h(mut self, h: impl IntoDimension) -> Self {
        self.layout_mut().max_size.height = h.into_dimension();
        self
    }
    fn min_h(mut self, h: impl IntoDimension) -> Self {
        self.layout_mut().min_size.height = h.into_dimension();
        self
    }
    fn wfill(mut self) -> Self {
        self.layout_mut().size.width = Dimension::percent(1.0);
        self
    }
    fn hfill(mut self) -> Self {
        self.layout_mut().size.height = Dimension::percent(1.0);
        self
    }
    fn fill(mut self) -> Self {
        self.layout_mut().size.width = Dimension::percent(1.0);
        self.layout_mut().size.height = Dimension::percent(1.0);
        self
    }
    fn cbox(mut self) -> Self {
        self.layout_mut().box_sizing = BoxSizing::ContentBox;
        self
    }
    fn grow(mut self, amount: f32) -> Self {
        self.layout_mut().flex_grow = amount;
        self
    }
}

pub trait Widget<'a>: std::fmt::Debug {
    fn layout(&self) -> TaffyLayout;
    fn style(&self) -> Option<Style> {
        None
    }
    fn node(&self) -> Option<usize> {
        None
    }
    //New retained layout
    #[track_caller]
    fn primitive(&self) -> Option<Primative> {
        unreachable!()
    }
    fn area_cell(&'a self) -> Option<&'a std::cell::Cell<Rect>> {
        None
    }
    fn draw_area(&'a self) -> Option<Size<f32>> {
        None
    }

    //TODO: Atomic mouse state to remove the &mut ctx
    fn clicked(&'a self, ctx: &mut Context) -> bool {
        if let Some(area) = self.area_cell() {
            clicked(ctx, area.get(), Left)
        } else {
            false
        }
    }
}

pub trait IntoDimension {
    fn into_dimension(self) -> Dimension;
}

impl<T: IntoF32> IntoDimension for T {
    #[inline(always)]
    fn into_dimension(self) -> Dimension {
        Dimension::length(self.into_f32())
    }
}

impl IntoDimension for Dimension {
    #[inline(always)]
    fn into_dimension(self) -> Dimension {
        self
    }
}

pub trait SimpleUnit {
    fn px(self) -> Dimension;
    fn percent(self) -> Dimension;
}

impl<T: IntoF32> SimpleUnit for T {
    #[inline(always)]
    fn px(self) -> Dimension {
        Dimension::length(self.into_f32())
    }

    #[inline(always)]
    fn percent(self) -> Dimension {
        Dimension::percent(self.into_f32() / 100.0)
    }
}

#[derive(Clone, Debug)]
pub struct WidgetData {
    pub area: std::cell::Cell<Rect>,
    pub layout: TaffyLayout,
    pub style: Style,
}
