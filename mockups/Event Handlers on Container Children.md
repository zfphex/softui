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