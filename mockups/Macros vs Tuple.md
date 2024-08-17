A very important aspect of this library is that error messages are clear and easy to understand.
This is not an easy task given Rust's lack of compile time reflection, specialisation and varidics.

Consider this error:

```rs
error[E0277]: the trait bound `(softui::Text<'_>, E): softui::Tuple` is not satisfied
   --> src/main.rs:79:19
    |
79  |                 v((text("hi"), E {}));
    |                 - ^^^^^^^^^^^^^^^^^^ the trait `softui::Tuple` is not implemented for `(softui::Text<'_>, E)`
    |                 |
    |                 required by a bound introduced by this call
    |
    = help: the following other types implement trait `softui::Tuple`:
              (V0, V1)
              (V0, V1, V2)
              (V0, V1, V2, V3)
              (V0, V1, V2, V3, V4)
              (V0, V1, V2, V3, V4, V5)
              (V0, V1, V2, V3, V4, V5, V6)
              (V0, V1, V2, V3, V4, V5, V6, V7)
              (V0, V1, V2, V3, V4, V5, V6, V7, V8)
            and 2 others
note: required by a bound in `softui::v`
   --> D:\desktop\projects\softui\src\widgets\layout.rs:285:19
    |
285 | pub const fn v<T: Tuple>(mut widgets: T) -> Container<T> {
    |                   ^^^^^ required by this bound in `v`

error[E0277]: the trait bound `(softui::Text<'_>, E): softui::Tuple` is not satisfied
   --> src/main.rs:79:17
    |
79  |                 v((text("hi"), E {}));
    |                 ^^^^^^^^^^^^^^^^^^^^^ the trait `softui::Tuple` is not implemented for `(softui::Text<'_>, E)`
    |
    = help: the following other types implement trait `softui::Tuple`:
              (V0, V1)
              (V0, V1, V2)
              (V0, V1, V2, V3)
              (V0, V1, V2, V3, V4)
              (V0, V1, V2, V3, V4, V5)
              (V0, V1, V2, V3, V4, V5, V6)
              (V0, V1, V2, V3, V4, V5, V6, V7)
              (V0, V1, V2, V3, V4, V5, V6, V7, V8)
            and 2 others
note: required by a bound in `Container`
   --> D:\desktop\projects\softui\src\widgets\layout.rs:330:25
    |
330 | pub struct Container<T: Tuple> {
    |                         ^^^^^ required by this bound in `Container`
```

This is very difficult to read, the first reason is the tuple implementations are very long and unclear, the user shouldn't really see any of this.
It's an unfortunate hack around the issues mentioned.

The second issue is here:

```rs
    |
79  |                 v((text("hi"), E {}));
    |                 ^^^^^^^^^^^^^^^^^^^^^ the trait `softui::Tuple` is not implemented for `(softui::Text<'_>, E)` 
```

Notice how the compiler doesn't tell you which of the two caused the error? I just says that `Tuple` is not implemented for `(softui::Text<'_>, E)`.

`E` is causing the issue here, but since it's all combined into one type(`T: Tuple`), the compiler has no idea which one is causing the issue.

Usually using the type system would give better errors than declarative macros. 
However this isn't the cause for this type of problem.

Let's look at an example:

```rs
error[E0277]: the trait bound `E: softui::Widget` is not satisfied
  --> src/main.rs:78:39
   |
70 |                         handle_widget($widget);
   |                         ------------- required by a bound introduced by this call
...
78 |                 vertical!(text("hi"), E {});
   |                                       ^^^^ the trait `softui::Widget` is not implemented for `E`
   |
   = help: the following other types implement trait `softui::Widget`:
             ()
             Container<T>
             RectangleNew
             softui::Rectangle<'a>
             softui::Text<'a>
note: required by a bound in `handle_widget`
  --> src/main.rs:64:37
   |
64 |             pub fn handle_widget<T: Widget>(widget: T) {}
   |                                     ^^^^^^ required by this bound in `handle_widget`
```

This is significally better, altough I will note a few problems. 

The function `handle_widget` is not known to the user when they call `vertical!`.

It is confusing to get errors inside the expanded macro.