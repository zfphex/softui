```rs
v(text("this is some text").on_clicked(|_| println!("do something")))

//Converts to

Text {}
fn on_clicked<F>(self, f: F) -> Text {
    if ... {
        f()
    }
    self
}

fn v(widgets: ...) {
    //Calculate the layout.
}
```

This is the wrong order because the area of a widget cannot be known until it's calculated.

This means it's impossible to use on_click since it checks for the intersection of the mouse and the widget area.

```rs
v(text("this is some text").on_clicked(|_| println!("do something")))

//Converts to

Text {}

enum WidgetType<W, F> {
    Clicked(W, F),
    Normal(W)
}

fn on_clicked<F>(self, f: F) -> OnClick<F> {
    WidgetType::Clicked(self, f)
}

fn v<T>(widgets: T) {
    widgets.for_each(|w| {
        match w {
            WidgetType::Clicked(_, _) => todo!(),
            WidgetType::Normal(_) => todo()!,
        }
    });
}
```

What about some other options?


```rs
enum Widget {
    Text(Text),
    Button(Button),
    Rectangle(Rectangle)
}

fn v<W: Into<Widget>>(widgets: &[W]) {
    for widget in widgets {
        match widget.into() {
            Widget::Text(text) => text.draw(),
            Widget::Button(button) => button.draw(),
            Widget::Button(rectangle) => rectangle.draw(),
        }
    }
}

struct CustomWidget {
    text: &str,
    prim: Rectangle,
}

impl Into<Widget> for CustomWidget {
    //
}

```

I don't think it's possible for users to extend the amount of widgets in this example.
If it is possible i think it would be a good choice.

Maybe there could be a primative widget type like `Rectangle`?

This doesn't seem right.

Basically I need a few funcitons, draw, clicked, area, and I need them to be on each widget.


```rs
trait Widget {
    fn draw();
    fn area();
    fn clicked();
}


//Works but ergonomics suck.
fn test(widgets: &[&dyn Widget]) {
    for w in widgets {
        dbg!(w.area());
    }
}

//Doesn't work, T must be the same widget :(.
fn test2<T: AsRef<dyn Widget>>(widgets: &[T]) {
    for w in widgets {
        dbg!(w.as_ref().area());
    }
}

test(&[&text("hi"), &rect()]);
test2(&[text("hi"), rect()]);
```


What about using macros instead?

Maybe with a proc macro we could get better error handling?
