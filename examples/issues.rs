use softui::*;

#[cfg(not(feature = "image"))]
fn main() {
    println!("Use --features 'image'")
}

#[cfg(feature = "image")]
fn main() {
    let _ctx = create_ctx("Softui", 800, 600);

    let mut _image = image("img/smol.jpg");

    while !matches!(_ctx.event(), Some(Event::Quit) | Some(Event::Input(Key::Escape, _))) {

        // This will not build
        // v!(_image.x(120));
    }
}
