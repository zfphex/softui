```rust
let mut state = 0;

button("I'm a button")
    .on_click(|_| state += 1)
    .on_lose_focus(|_| state = 0)
```

```log
error[E0499]: cannot borrow `state` as mutable more than once at a time
  --> examples/closure.rs:15:28
   |
14 |             .on_click(Left, |_| state += 1)
   |                             --- ----- first borrow occurs due to use of `state` in closure
   |                             |
   |                             first mutable borrow occurs here
15 |             .on_lose_focus(|_| state += 1)
   |              ------------- ^^^ ----- second borrow occurs due to use of `state` in closure
   |              |             |
   |              |             second mutable borrow occurs here
   |              first borrow later used by call
```
