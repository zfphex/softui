```
error: the `behaviour` method cannot be invoked on a trait object
  --> examples\dynamic.rs:17:21
   |
8  |         Self: Sized,
   |               ----- this has a `Sized` requirement
...
17 |         dbg!(widget.behaviour());
   |                     ^^^^^^^^^
```

```rust

use softui::*;

pub trait DynWidget {
    fn area(&self) -> Rect;
    fn primative(&self) -> Primative;
    fn behaviour(&mut self) -> Option<&mut Vec<Click<Self>>>
    where
        Self: Sized,
    {
        None
    }
}

fn widgets<const N: usize>(widgets: [&mut dyn DynWidget; N]) {
    for widget in widgets {
        dbg!(widget.area());
        dbg!(widget.behaviour());
    }
}

struct Test {}

impl DynWidget for Test {
    fn area(&self) -> Rect {
        Rect::new(0, 0, 0, 0)
    }

    fn primative(&self) -> Primative {
        todo!()
    }
}

fn main() {
    widgets([&mut Test {}, &mut Test {}])
}
```
