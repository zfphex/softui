use crate::*;

impl IntoNode for &str {
    fn into_node(self) -> usize {
        text(self).into_node()
    }
}

pub trait StylingNew: Sized {
    type Return = Self;
    fn bg(self, bg: impl IntoColor) -> Self::Return;
    fn fg(self, fg: impl IntoColor) -> Self::Return;
}

impl<'a> StylingNew for &'a str {
    type Return = Text<'a>;

    fn bg(self, bg: impl IntoColor) -> Self::Return {
        text(self).bg(bg)
    }

    fn fg(self, fg: impl IntoColor) -> Self::Return {
        text(self).fg(fg)
    }
}
