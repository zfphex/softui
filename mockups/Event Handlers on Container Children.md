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
struct Text {
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