- It is probably not a good idea to use objc-sys, the complexity is too high and it's not safe anyway
- A simple wrapper written in obj-c is probably the best idea.
- Can use C abi to interface with rust.
- Might be multiple conversion steps when translating events.
- https://github.com/emoon/rust_minifb/blob/master/build.rs

```rs
 let env = env::var("TARGET").unwrap();
 if env.contains("darwin") {
        cc::Build::new()
            .flag("-mmacosx-version-min=10.10")
            .file("src/native/macosx/MacMiniFB.m")
            .file("src/native/macosx/OSXWindow.m")
            .file("src/native/macosx/OSXWindowFrameView.m")
            .compile("libminifb_native.a");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
    }
```
