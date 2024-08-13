Consider the example:

```rs
let container = v((text("first box"), text("second box").on_clicked(|_| println!("Second box clicked"))))
//                                                       ^ this function is called first

//Then container is dropped
drop(container)
//^ container.draw() called here.

Container {
    widgets: (Text, Text)
}
```

Here the `on_clicked` function is called before the layout is calculated.
The layout is calculated when the container is drawn.


To fix this, on_click must be stored and defered until the widget is drawn.

```rs
struct Text<F: FnMut(&mut Self)> {
    on_clicked: F,
}

pub fn draw(&self) {
    //Calculate the layout

    for widget in self.widgets {
        widget.draw();
        widget.on_clicked();
    }
}
```

The logistics of this seems pretty awful. Every single widget would need to include a generic parameter.

There is also the issue of [cyclic closures](https://github.com/rust-lang/rust/issues/46062). 

Since the widget takes a reference to itself, this will not compile. 

Other options are `Box<dyn FnMut(&mut Text)>`, but that won't work because `Box` must have a `'static` lifetime.
This means that any of the captured variables of must also be `'static`, altough they are all `'a`, they are not `'static`

The main problem is the way layout is calculated. Since the calculation is done after the even handler would typically be triggerd.
The closure must be stored and executed later, however since it's executed later, the captured variables are _sorta_ outside of scope.

```rs
let state = 0;

{
    let container = container(widget().on_clicked(|_| state += 10))

    //contaier.draw() called here
    drop(container);

    fn draw() {
        for widget in container {
            //state modified here.
            widget.on_clicked();
        }
    }
}
```


One of the alteratives is that every widget must hold all of it's own state. This is awful since you would not be able to acess state from other widgets.

```rs
struct ... {
    clicked: fn(&mut Self),
}
```

One work around for this is that every single ui element is contained in the same struct. I _think_ this is the way xilem works. 

```rs
struct App {
    button: Button,
}

// ----------vs-----------

let button: Button::new();

loop {
    //handle the state
}

```

# Possible Fix


Typically you would need to something like this.

```rs
v(|ui| {
    ui.text("a");
    ui.text("b");
}).gap(12);
```

However something like this might also work.
```rs
gap(12).v(|| {
    text("a"); //Layout is calculated using immediately.
    text("b");
});

fn v() {
    LAYOUT = ...
}

fn text() {
    //Skip passing in the layout.
    let layout = &LAYOUT;

    //... text stuff
}
```

This has a number of issues, the first is thread safety. All round feels like a bad idea. Plus I don't like the prefix `gap()`.

Another idea

```rs
struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

struct Layout {}

impl Layout {
    pub fn clicked(&self) -> bool {
        true
    }
}

//T would be tuple here.
fn vertical<T>(w1: impl AsMut<T>, w2: impl AsMut<T>) -> Layout {
    //Calcaulte the layout.
    let layout = Rect {
        x: 0,
        y: 0,
        width: 30, //Example values
        height: 10,
        // padding: 0,
        // marign: 0,
    };

    // w1.area = ...
    // w2.area = ...

    // draw text

    Layout {}
}

struct Style {}

struct Text<'a> {
    content: &'a str,
    area: Rect,
    style: Style,
}

impl<'a> Text<'a> {
    pub fn clicked(&self) -> bool {
        true
    }
}

impl<'a> AsMut<Text<'a>> for Text<'a> {
    fn as_mut(&mut self) -> &mut Text<'a> {
        self
    }
}

fn text(content: &str) -> Text {
    Text {
        content,
        area: Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        style: Style {},
    }
}

fn main() {
    // let mut text1 = text("this is some text");
    let mut text2 = text("this is some more text");

    //Area modified here.
    let v = vertical(text("this is some text"), &mut text2);

    //Clicked will have use the correct text area, since the layout was just calculated.
    if text2.clicked() {
        println!("Do something")
    }

    //This works too.
    if v.clicked() {}
}
```