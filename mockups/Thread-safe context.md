I would like to stop using `Context` in the global scope, `ctx()` will not work in a multithreaded context.

It should be changed to something like this:

```rs
static mut AREA: AtomicRect = AtomicRect::new();
//or
static mut WIDTH: AtomicUsize = AtomicUsize::new();
static mut HEIGHT: AtomicUsize = AtomicUsize::new();

#[inline]
fn canvas_width() -> usize {
    unsafe { WIDTH.load(Ordering::Relaxed) }
}

#[inline]
fn canvas_height() -> usize {
    unsafe { HEIGHT.load(Ordering::Relaxed) }
}

//or

#[inline]
fn canvas_area() -> Rect {
    unsafe { AREA.load(Ordering::Relaxed) }
}

#[inline]
fn canvas_width() -> usize {
    unsafe { AREA.load(Ordering::Relaxed).width as usize}
}

#[inline]
fn canvas_height() -> usize {
    unsafe { AREA.load(Ordering::Relaxed).width as usize}
}

//fn draw()....

if self.area != area {
    unsafe { AREA.store(area, Ordering::Relaxed) };
    //or
    unsafe { WIDTH.store(area.width, Ordering::Relaxed) };
    unsafe { HEIGHT.store(area.height, Ordering::Relaxed) };

    self.area = area;
    self.buffer.clear();
    self.buffer
        .resize(self.area.width as usize * self.area.height as usize, 0);
    self.bitmap = BITMAPINFO::new(self.area.width as i32, self.area.height as i32);
}

//...

fn custom_widget() {
    let width = canvas_width();
    //...
}
```
