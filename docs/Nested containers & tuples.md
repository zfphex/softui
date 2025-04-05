The current layout system I wrote works okay, but doesn't work with nested containers.
It also strips the type information which destroys the ability to use the click function.

Click was awful to begin with but I'm lost for choice okay...

```
rect().clicked(|r| println!("{}", r.area))
```

Every single widget needs to store a closure or every click needs to store a widget and a closure.

the latter is obviously the most ergonomic, but the languages lack of variadic support makes the implementation awful.
