```rs
fn test() -> impl FnMut(&mut Text) -> () {
    |text: &mut Text| {}
}

let mut test = || {
    println!("width: {}", ctx.area.width);
    return 2;
};
let test_ptr = addr_of_mut!(test);
let t: fn() -> i32 = unsafe { transmute_copy(&(test_ptr as u128)) };
let t = unsafe { test_ptr.as_mut().unwrap() };
let x = t();
```
