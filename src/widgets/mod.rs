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
use taffy::Dimension;
pub use text::*;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub mod dwrite;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub use dwrite::*;

pub mod generic;
pub use generic::*;

use crate::*;

pub trait Widget<'a>: std::fmt::Debug {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>);
    fn layout(&self) -> TaffyLayout;
    fn style(&self) -> Option<Style> {
        None
    }
    fn is_container(&self) -> bool {
        false
    }
    fn node(&self) -> usize {
        unreachable!()
    }
    fn fg(self, fg: impl IntoColor) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).fg(fg)
    }
    fn bg(self, bg: impl IntoColor) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).bg(bg)
    }
    fn w(self, w: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).w(w)
    }
    fn h(self, h: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).h(h)
    }
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
    fn wh(self, wh: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).wh(wh)
    }
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
    fn pad(self, pad: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pad(pad)
    }
    fn pl(self, left: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pl(left)
    }
    fn pr(self, right: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pr(right)
    }
    fn pt(self, top: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pt(top)
    }
    fn pb(self, bottom: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pb(bottom)
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
    fn try_click(&mut self, _: &mut Context, _: Rect) {}
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
