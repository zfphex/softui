```rs

vertical((
    text("text 1").on_click(|_| println!("Clicked on text"),
    text("text 2").on_click(|_| println!("Clicked on text 2"),
))

struct OnClickFunction<T: Widget, F: FnMut(&mut T)> {
    f: F,
    button: MouseButton,
}

struct OnClick<T: Widget, F: FnMut(&mut T)> {
    widget: T,
    on_click: Option<OnClickFunction<T, F>>,
}

T is type (OnClick<Text, FnMut(&mut Text)>, OnClick<Text, FnMut(&mut Text)>)

fn vertical<T: Tuple>(widgets: T) {
}
```
