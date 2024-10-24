In order to render a image across threads it's lifetime must be longer than the threads.

```rs
let image = image("img/smol.png");

thread::spawn(|| {
    v!(&image);
});

thread::spawn(|| {
    v!(&image);
});
```

Honestly the main problem is that the layout system must either always take in a reference or never.


