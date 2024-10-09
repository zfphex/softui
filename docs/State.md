```rs

vertical((
    text("text 1").on_click(|_| println!("Clicked on text"),
    text("text 2").on_click(|_| println!("Clicked on text 2"),
))

struct OnClick<T: Widget, F: FnMut(&mut T)> {
    widget: T,
    on_click: Option<(F, MouseButton)>,
}

T is type (OnClick<Text, FnMut(&mut Text)>, OnClick<Text, FnMut(&mut Text)>)

fn vertical<T: Tuple>(widgets: T) {
}
```

```rs

vertical((
    text("text 1").on_click(|_| println!("Clicked on text"),
    text("text 2").on_click(|_| println!("Clicked on text 2"),
))

struct OnClick<T: Widget> {
    widget: T,
    //just a pointer.
    on_click: usize
    button: MouseButton,
}


T is type (OnClick<Text>, OnClick<Text>)

fn vertical<T: Tuple>(widgets: T) {
}
```
