```rs
pub struct Click0<T: Widget, F0: FnMut(&mut T)> {
    pub widget: T,
    pub click: ((MouseButton, F0),),
}
impl<T: Widget, F0: FnMut(&mut T)> Click0<T, F0> {
    pub fn on_click<F1: FnMut(&mut T)>(self, button: MouseButton, f: F1) -> Click1<T, F0, F1> {
        Click1 {
            widget: self.widget,
            click: (self.click.0, (button, f)),
        }
    }
}
impl<T: Widget, F0: FnMut(&mut T)> Widget for Click0<T, F0> {
    #[inline]
    fn area(&mut self) -> Option<&mut super::Rect> {
        self.widget.area()
    }
    #[inline]
    fn layout_area(&mut self) -> Option<&mut super::Rect> {
        self.widget.layout_area()
    }
    #[inline]
    fn try_click(&mut self) {
        let ctx = ctx();
        if clicked(ctx, &mut self.widget, self.click.0 .0) {
            (self.click.0 .1)(&mut self.widget);
        }
    }
    #[inline]
    fn draw_command(&self) -> Option<Command> {
        self.widget.draw_command()
    }
}
impl<T: Widget, F0: FnMut(&mut T)> std::fmt::Debug for Click0<T, F0> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Click0")
            .field("widget", &self.widget)
            .finish()
    }
}
pub struct Click1<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T)> {
    pub widget: T,
    pub click: ((MouseButton, F0), (MouseButton, F1)),
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T)> Click1<T, F0, F1> {
    pub fn on_click<F2: FnMut(&mut T)>(self, button: MouseButton, f: F2) -> Click2<T, F0, F1, F2> {
        Click2 {
            widget: self.widget,
            click: (self.click.0, self.click.1, (button, f)),
        }
    }
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T)> Widget for Click1<T, F0, F1> {
    #[inline]
    fn area(&mut self) -> Option<&mut super::Rect> {
        self.widget.area()
    }
    #[inline]
    fn layout_area(&mut self) -> Option<&mut super::Rect> {
        self.widget.layout_area()
    }
    #[inline]
    fn try_click(&mut self) {
        let ctx = ctx();
        if clicked(ctx, &mut self.widget, self.click.0 .0) {
            (self.click.0 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.1 .0) {
            (self.click.1 .1)(&mut self.widget);
        }
    }
    #[inline]
    fn draw_command(&self) -> Option<Command> {
        self.widget.draw_command()
    }
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T)> std::fmt::Debug for Click1<T, F0, F1> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Click0")
            .field("widget", &self.widget)
            .finish()
    }
}
pub struct Click2<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T), F2: FnMut(&mut T)> {
    pub widget: T,
    pub click: ((MouseButton, F0), (MouseButton, F1), (MouseButton, F2)),
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T), F2: FnMut(&mut T)> Click2<T, F0, F1, F2> {
    pub fn on_click<F3: FnMut(&mut T)>(
        self,
        button: MouseButton,
        f: F3,
    ) -> Click3<T, F0, F1, F2, F3> {
        Click3 {
            widget: self.widget,
            click: (self.click.0, self.click.1, self.click.2, (button, f)),
        }
    }
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T), F2: FnMut(&mut T)> Widget
    for Click2<T, F0, F1, F2>
{
    #[inline]
    fn area(&mut self) -> Option<&mut super::Rect> {
        self.widget.area()
    }
    #[inline]
    fn layout_area(&mut self) -> Option<&mut super::Rect> {
        self.widget.layout_area()
    }
    #[inline]
    fn try_click(&mut self) {
        let ctx = ctx();
        if clicked(ctx, &mut self.widget, self.click.0 .0) {
            (self.click.0 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.1 .0) {
            (self.click.1 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.2 .0) {
            (self.click.2 .1)(&mut self.widget);
        }
    }
    #[inline]
    fn draw_command(&self) -> Option<Command> {
        self.widget.draw_command()
    }
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T), F2: FnMut(&mut T)> std::fmt::Debug
    for Click2<T, F0, F1, F2>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Click0")
            .field("widget", &self.widget)
            .finish()
    }
}
pub struct Click3<
    T: Widget,
    F0: FnMut(&mut T),
    F1: FnMut(&mut T),
    F2: FnMut(&mut T),
    F3: FnMut(&mut T),
> {
    pub widget: T,
    pub click: (
        (MouseButton, F0),
        (MouseButton, F1),
        (MouseButton, F2),
        (MouseButton, F3),
    ),
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T), F2: FnMut(&mut T), F3: FnMut(&mut T)>
    Click3<T, F0, F1, F2, F3>
{
    pub fn on_click<F4: FnMut(&mut T)>(
        self,
        button: MouseButton,
        f: F4,
    ) -> Click4<T, F0, F1, F2, F3, F4> {
        Click4 {
            widget: self.widget,
            click: (
                self.click.0,
                self.click.1,
                self.click.2,
                self.click.3,
                (button, f),
            ),
        }
    }
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T), F2: FnMut(&mut T), F3: FnMut(&mut T)> Widget
    for Click3<T, F0, F1, F2, F3>
{
    #[inline]
    fn area(&mut self) -> Option<&mut super::Rect> {
        self.widget.area()
    }
    #[inline]
    fn layout_area(&mut self) -> Option<&mut super::Rect> {
        self.widget.layout_area()
    }
    #[inline]
    fn try_click(&mut self) {
        let ctx = ctx();
        if clicked(ctx, &mut self.widget, self.click.0 .0) {
            (self.click.0 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.1 .0) {
            (self.click.1 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.2 .0) {
            (self.click.2 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.3 .0) {
            (self.click.3 .1)(&mut self.widget);
        }
    }
    #[inline]
    fn draw_command(&self) -> Option<Command> {
        self.widget.draw_command()
    }
}
impl<T: Widget, F0: FnMut(&mut T), F1: FnMut(&mut T), F2: FnMut(&mut T), F3: FnMut(&mut T)>
    std::fmt::Debug for Click3<T, F0, F1, F2, F3>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Click0")
            .field("widget", &self.widget)
            .finish()
    }
}
pub struct Click4<
    T: Widget,
    F0: FnMut(&mut T),
    F1: FnMut(&mut T),
    F2: FnMut(&mut T),
    F3: FnMut(&mut T),
    F4: FnMut(&mut T),
> {
    pub widget: T,
    pub click: (
        (MouseButton, F0),
        (MouseButton, F1),
        (MouseButton, F2),
        (MouseButton, F3),
        (MouseButton, F4),
    ),
}
impl<
        T: Widget,
        F0: FnMut(&mut T),
        F1: FnMut(&mut T),
        F2: FnMut(&mut T),
        F3: FnMut(&mut T),
        F4: FnMut(&mut T),
    > Click4<T, F0, F1, F2, F3, F4>
{
    pub fn on_click<F5: FnMut(&mut T)>(
        self,
        button: MouseButton,
        f: F5,
    ) -> Click5<T, F0, F1, F2, F3, F4, F5> {
        Click5 {
            widget: self.widget,
            click: (
                self.click.0,
                self.click.1,
                self.click.2,
                self.click.3,
                self.click.4,
                (button, f),
            ),
        }
    }
}
impl<
        T: Widget,
        F0: FnMut(&mut T),
        F1: FnMut(&mut T),
        F2: FnMut(&mut T),
        F3: FnMut(&mut T),
        F4: FnMut(&mut T),
    > Widget for Click4<T, F0, F1, F2, F3, F4>
{
    #[inline]
    fn area(&mut self) -> Option<&mut super::Rect> {
        self.widget.area()
    }
    #[inline]
    fn layout_area(&mut self) -> Option<&mut super::Rect> {
        self.widget.layout_area()
    }
    #[inline]
    fn try_click(&mut self) {
        let ctx = ctx();
        if clicked(ctx, &mut self.widget, self.click.0 .0) {
            (self.click.0 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.1 .0) {
            (self.click.1 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.2 .0) {
            (self.click.2 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.3 .0) {
            (self.click.3 .1)(&mut self.widget);
        }
        if clicked(ctx, &mut self.widget, self.click.4 .0) {
            (self.click.4 .1)(&mut self.widget);
        }
    }
    #[inline]
    fn draw_command(&self) -> Option<Command> {
        self.widget.draw_command()
    }
}
impl<
        T: Widget,
        F0: FnMut(&mut T),
        F1: FnMut(&mut T),
        F2: FnMut(&mut T),
        F3: FnMut(&mut T),
        F4: FnMut(&mut T),
    > std::fmt::Debug for Click4<T, F0, F1, F2, F3, F4>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Click0")
            .field("widget", &self.widget)
            .finish()
    }
}
pub struct Click5<
    T: Widget,
    F0: FnMut(&mut T),
    F1: FnMut(&mut T),
    F2: FnMut(&mut T),
    F3: FnMut(&mut T),
    F4: FnMut(&mut T),
    F5: FnMut(&mut T),
> {
    pub widget: T,
    pub click: (
        (MouseButton, F0),
        (MouseButton, F1),
        (MouseButton, F2),
        (MouseButton, F3),
        (MouseButton, F4),
        (MouseButton, F5),
    ),
}
impl<
        T: Widget,
        F0: FnMut(&mut T),
        F1: FnMut(&mut T),
        F2: FnMut(&mut T),
        F3: FnMut(&mut T),
        F4: FnMut(&mut T),
        F5: FnMut(&mut T),
    > Click5<T, F0, F1, F2, F3, F4, F5>
{
    pub fn on_click<F6: FnMut(&mut Self)>(self, button: MouseButton, f: F6) {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!(
                        "On click chains are only support up to 5 times. Left, Right, Middle, Forward, Back.",
                    ),
                ),
            );
        };
    }
}
```
