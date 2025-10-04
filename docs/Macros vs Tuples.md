# Macros vs Tuples

One of the most important aspect of a UI library is the error messages, they should be clear and easy to understand.
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

This is very difficult to read, the first reason is the tuple implementation (cool pyramid though). 

The second issue is here:

```rs
    |
79  |                 v((text("hi"), E {}));
    |                 ^^^^^^^^^^^^^^^^^^^^^ the trait `softui::Tuple` is not implemented for `(softui::Text<'_>, E)` 
```

Notice how the compiler doesn't tell you which of the two caused the error? I just says that `Tuple` is not implemented for `(softui::Text<'_>, E)`.

`E` is causing the issue here, but since a tuple is multiple tupes combined into one type(`T: Tuple`), the compiler has no idea which one is causing the issue.

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

----

I love [xilem](https://github.com/linebender/xilem) but, the errors are even worse.

```rs
error[E0277]: the trait bound `(E, xilem::view::FlexSpacer, impl WidgetView<i32>, xilem::view::FlexSpacer, FlexItem<xilem::view::Label, _, _>, xilem::view::FlexSpacer, impl WidgetView<i32>, xilem::view::FlexSpacer): FlexSequence<_, _>` is not satisfied
  --> xilem\examples\flex.rs:25:10
   |
25 |       flex((
   |  _____----_^
   | |     |
   | |     required by a bound introduced by this call
26 | |         E {},
27 | |         FlexSpacer::Fixed(30.0),
28 | |         big_button("-", |data| {
...  |
37 | |         FlexSpacer::Fixed(30.0),
38 | |     ))
   | |_____^ the trait `ViewSequence<_, _, ViewCtx, FlexElement>` is not implemented for `(E, FlexSpacer, impl WidgetView<i32>, FlexSpacer, FlexItem<Label, _, _>, FlexSpacer, impl WidgetView<i32>, FlexSpacer)`, which is required by `(E, xilem::view::FlexSpacer, impl WidgetView<i32>, xilem::view::FlexSpacer, FlexItem<xilem::view::Label, _, _>, xilem::view::FlexSpacer, impl WidgetView<i32>, xilem::view::FlexSpacer): FlexSequence<_, _>`
   |
   = help: the following other types implement trait `ViewSequence<State, Action, Context, Element, Message>`:
             ()
             (Seq,)
             (Seq0, Seq1)
             (Seq0, Seq1, Seq2)
             (Seq0, Seq1, Seq2, Seq3)
             (Seq0, Seq1, Seq2, Seq3, Seq4)
             (Seq0, Seq1, Seq2, Seq3, Seq4, Seq5)
             (Seq0, Seq1, Seq2, Seq3, Seq4, Seq5, Seq6)
           and 9 others
   = note: required for `(E, FlexSpacer, impl WidgetView<i32>, FlexSpacer, FlexItem<Label, _, _>, FlexSpacer, impl WidgetView<i32>, FlexSpacer)` to implement `FlexSequence<_, _>`
note: required by a bound in `xilem::view::flex`
  --> D:\Desktop\xilem\xilem\src\view\flex.rs:19:33
   |
19 | pub fn flex<State, Action, Seq: FlexSequence<State, Action>>(
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `flex`
   = note: the full name for the type has been written to D:\Desktop\xilem\target\debug\examples\flex.long-type-3082561552280108783.txt
   = note: consider using `--verbose` to print the full type name to the console

error[E0277]: the trait bound `(E, xilem::view::FlexSpacer, impl WidgetView<i32>, xilem::view::FlexSpacer, FlexItem<xilem::view::Label, _, _>, xilem::view::FlexSpacer, impl WidgetView<i32>, xilem::view::FlexSpacer): ViewSequence<i32, (), ViewCtx, FlexElement>` is not satisfied
  --> xilem\examples\flex.rs:24:33
   |
24 | fn app_logic(data: &mut i32) -> impl WidgetView<i32> {
   |                                 ^^^^^^^^^^^^^^^^^^^^ the trait `ViewSequence<i32, (), ViewCtx, FlexElement>` is not implemented for `(E, FlexSpacer, impl WidgetView<i32>, FlexSpacer, FlexItem<Label, _, _>, FlexSpacer, impl WidgetView<i32>, FlexSpacer)`, which is required by `xilem::view::Flex<(E, xilem::view::FlexSpacer, impl WidgetView<i32>, xilem::view::FlexSpacer, FlexItem<xilem::view::Label, _, _>, xilem::view::FlexSpacer, impl WidgetView<i32>, xilem::view::FlexSpacer), _, _>: WidgetView<i32>`
   |
   = help: the following other types implement trait `ViewSequence<State, Action, Context, Element, Message>`:
             ()
             (Seq,)
             (Seq0, Seq1)
             (Seq0, Seq1, Seq2)
             (Seq0, Seq1, Seq2, Seq3)
             (Seq0, Seq1, Seq2, Seq3, Seq4)
             (Seq0, Seq1, Seq2, Seq3, Seq4, Seq5)
             (Seq0, Seq1, Seq2, Seq3, Seq4, Seq5, Seq6)
           and 9 others
   = note: required for `(E, FlexSpacer, impl WidgetView<i32>, FlexSpacer, FlexItem<Label, _, _>, FlexSpacer, impl WidgetView<i32>, FlexSpacer)` to implement `FlexSequence<i32>`   = note: required for `Flex<(E, FlexSpacer, impl WidgetView<i32>, FlexSpacer, FlexItem<Label, _, _>, FlexSpacer, impl WidgetView<i32>, FlexSpacer), i32>` to implement `View<i32, (), ViewCtx>`
   = note: required for `Flex<(E, FlexSpacer, impl WidgetView<i32>, FlexSpacer, FlexItem<Label, _, _>, FlexSpacer, impl WidgetView<i32>, FlexSpacer), i32>` to implement `WidgetView<i32>`
   = note: the full name for the type has been written to D:\Desktop\xilem\target\debug\examples\flex.long-type-12281210820452562655.txt
   = note: consider using `--verbose` to print the full type name to the console
   = note: the full name for the type has been written to D:\Desktop\xilem\target\debug\examples\flex.long-type-12281210820452562655.txt
   = note: consider using `--verbose` to print the full type name to the console
   = note: the full name for the type has been written to D:\Desktop\xilem\target\debug\examples\flex.long-type-2266967692064486613.txt
   = note: consider using `--verbose` to print the full type name to the console

For more information about this error, try `rustc --explain E0277`.
error: could not compile `xilem` (example "flex") due to 2 previous errors
```

The type is so long that it had to be writen to a file...