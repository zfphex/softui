# Containers

```rs
v((text("a"), text("b")))

Container<T: Tuple> {
    widgets: T
    direction: Vertical
}

T {
    (Text {text: "a"}, Text {text: "b"})
}

fn draw() {
    //We don't have iterator for the tuple trait yet.
    for widget in widgets {
        //We can't draw the widgets yet.
        //We must lay them out correctly.
    }

    for widget in widgets {
        //Draw the widgets once they have been layed out.
    }
}

```

# Closure Containers


```rs

v(|ui|{ 
    button(ui, "a");
    button(ui, "b");
    //or
    ui.button("button a");
    ui.button("button b");
})

``` 