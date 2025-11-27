Currently widgets cannot hold their own state since the library is immediate mode.
This makes ergonomics quite bad for users trying to create input boxes and buttons.

To fix this, we can store the focused state inside of every node. That way the user doesn't need to do anything to know if it's an active or selected widget.

Reminder to set `SetCapture(hwnd)` to allow out of window drags to still be processed.

```rs
//Only used with on_click()
enum MouseButton {
    Left,
    Right,
}

//Used with on_press() and on_released()
enum Button {
    LeftMouse,
    Space,
    W,
}
```

Then

```rs

rect().on_press(Space, |_| {})

//Should do something like this.
state = GetAsyncKeyState(key.into_win32())

```
