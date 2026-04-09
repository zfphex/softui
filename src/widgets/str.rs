use crate::*;

impl IntoNode for &str {
    fn into_node(self) -> usize {
        text(self).into_node()
    }
}

pub trait StylingNew: Sized {
    type Return = Self;
    fn bg(self, bg: Option<u32>) -> Self::Return;
    fn fg(self, fg: Option<u32>) -> Self::Return;
}

impl<'a> StylingNew for &'a str {
    type Return = Text<'a>;

    fn bg(self, bg: Option<u32>) -> Self::Return {
        text(self).bg(bg)
    }

    fn fg(self, fg: Option<u32>) -> Self::Return {
        text(self).fg(fg)
    }
}
