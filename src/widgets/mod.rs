pub mod rectangle;
pub use rectangle::*;

pub mod list;
pub use list::*;

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "svg")]
pub use svg::*;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "image")]
pub use image::*;

pub mod text;
use taffy::{prelude::length, AvailableSpace, Dimension, Size};
pub use text::*;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub mod dwrite;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub use dwrite::*;

pub mod generic;
pub use generic::*;

pub mod input;
pub use input::*;

pub mod button;
pub use button::*;

pub mod text_new;
pub use text_new::*;

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
}

pub trait Sizing: Sized {
    fn layout_mut(&mut self) -> &mut TaffyLayout;

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
}

pub trait Widget<'a>: std::fmt::Debug {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>);
    fn layout(&self) -> TaffyLayout;

    fn measure(&self, known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
        Size::ZERO
    }
    fn style(&self) -> Option<Style> {
        None
    }
    fn is_container(&self) -> bool {
        false
    }
    fn node(&self) -> usize {
        unreachable!()
    }
    // fn fg(self, fg: impl IntoColor) -> GenericWidget<'a, Self>
    // where
    //     Self: Sized,
    // {
    //     GenericWidget::new(self).fg(fg)
    // }
    // fn bg(self, bg: impl IntoColor) -> GenericWidget<'a, Self>
    // where
    //     Self: Sized,
    // {
    //     GenericWidget::new(self).bg(bg)
    // }
    // fn border(self, border: impl IntoColor) -> GenericWidget<'a, Self>
    // where
    //     Self: Sized,
    // {
    //     GenericWidget::new(self).border(border)
    // }

    // TODO: I don't think that width and height should necessarily be changed on some widgets.
    // The width and height information for font is really important.
    // Destroying it doesn't really make sense since you can just set constraints instead?
    // idk. I had some issues with
    // fit!(text(plan).w(20))
    // vs.
    // fit!(text(plan.w(20)))
    // they have different behaviour and I don't like that 😡

    // fn w(self, w: impl IntoDimension) -> GenericWidget<'a, Self>
    // where
    //     Self: Sized,
    // {
    //     GenericWidget::new(self).w(w)
    // }
    // fn h(self, h: impl IntoDimension) -> GenericWidget<'a, Self>
    // where
    //     Self: Sized,
    // {
    //     GenericWidget::new(self).h(h)
    // }

    fn max_w(self, w: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).max_w(w)
    }
    fn min_w(self, w: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).min_w(w)
    }
    fn max_h(self, h: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).max_h(h)
    }
    fn min_h(self, h: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).min_h(h)
    }
    // fn wh(self, wh: impl IntoDimension) -> GenericWidget<'a, Self>
    // where
    //     Self: Sized,
    // {
    //     GenericWidget::new(self).wh(wh)
    // }
    fn fit(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).fit()
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
    fn fill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).fill()
    }
    fn margin(self, margin: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).margin(margin)
    }
    fn ml(self, left: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).ml(left)
    }
    fn mr(self, right: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).mr(right)
    }
    fn mt(self, top: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).mt(top)
    }
    fn mb(self, bottom: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).mb(bottom)
    }
    fn on_hover<F>(self, func: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_hover(func)
    }
    fn on_click<F>(self, button: MouseButton, func: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_click(button, func)
    }
    fn on_press<F>(self, button: MouseButton, func: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_press(button, func)
    }
    fn on_release<F>(self, button: MouseButton, func: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_release(button, func)
    }
    fn try_click(&mut self, ctx: &mut Context, area: Rect) {}
    fn into_layout(self) -> TaffyLayout
    where
        Self: Sized,
    {
        GenericWidget::new(self).into_layout()
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
