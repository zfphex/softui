There is no way to hit-test a widget immediately after creation.
Widgets must first go through a layout-pass to calculate there position.

Be definition any input will always be one frame behind. How could you the user click on something that hasn't been rendered yet? This means input hadling must happen after the user-interface is rendered.

I think a common strategy to cache the previous frame (all widget size and position data) and then use that for hit testing.

Imagine the user clicks sometime between the zero-th and first frame. There has not been anything rendered yet so the user cannot click on anything.

Currently we click after the new layout has been calculated so it doesn't actually reflect what the user saw. Not that it really matters, but techinally the previous method would be more accurate.
More importantly the new method allows for code to be written like this:

```rust
//Uses the previous frame's layout metrics to calculate if the widget has been clicked immediately.
//It is not deffered through a closure and therefore lifetime issues don't present themselves.
if text("Button1").clicked() {
    println!("Clicked!")
}
```
