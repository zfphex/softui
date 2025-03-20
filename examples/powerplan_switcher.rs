#![allow(unused)]

use softui::*;

const GUID_MAX_POWER_SAVINGS: GUID = GUID::from_u128(0xa1841308_3541_4fab_bc81_f71556f20b4a);
const GUID_MIN_POWER_SAVINGS: GUID = GUID::from_u128(0x8c5e7fda_e8bf_4a96_9a85_a6e23a8c635c);
const GUID_TYPICAL_POWER_SAVINGS: GUID = GUID::from_u128(0x381b4222_f694_41f0_9685_ff5bb260df2e);

#[link(name = "powrprof")]
unsafe extern "system" {
    pub fn PowerSetActiveScheme(UserRootPowerKey: *mut c_void, SchemeGuid: *const GUID) -> u32;
    pub fn PowerGetActiveScheme(UserRootPowerKey: *mut c_void, ActivePolicyGuid: *mut *mut GUID) -> u32;
    //TODO: Move into window, could be useful for users.
    pub fn DwmGetColorizationColor(pcrColorization: *mut DWORD, pfOpaqueBlend: *mut BOOL) -> i32;
}

fn high_performance() {
    unsafe { PowerSetActiveScheme(core::ptr::null_mut(), &GUID_MIN_POWER_SAVINGS) };
}

fn balanced() {
    unsafe { PowerSetActiveScheme(core::ptr::null_mut(), &GUID_TYPICAL_POWER_SAVINGS) };
}

fn power_saver() {
    unsafe { PowerSetActiveScheme(core::ptr::null_mut(), &GUID_MAX_POWER_SAVINGS) };
}

fn current_plan() -> &'static str {
    unsafe {
        let mut policy = core::mem::zeroed();
        assert!(PowerGetActiveScheme(core::ptr::null_mut(), &mut policy) == 0);
        assert!(!policy.is_null());
        match *policy {
            GUID_MAX_POWER_SAVINGS => "Power saver",
            GUID_MIN_POWER_SAVINGS => "High performance",
            GUID_TYPICAL_POWER_SAVINGS => "Balanced",
            _ => unreachable!(),
        }
    }
}

fn accent_color() -> Color {
    unsafe {
        let mut color = core::mem::zeroed();
        let mut blend = core::mem::zeroed();
        assert!(DwmGetColorizationColor(&mut color, &mut blend) == 0);
        let r = (color & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = ((color >> 16) & 0xFF) as u8;
        Color::new(b, g, r)
    }
}

fn main() {
    let padding = 10;
    let font_size = 20;
    let rect_height = 30;

    let width = 360;
    let height = 3 * (font_size as i32 + padding as i32);

    let window = create_window(
        "Power Plan Switcher",
        1920 - width,
        1080 - height,
        width,
        height,
        WindowStyle::BORDERLESS.ex_style(WS_EX_TOPMOST | WS_EX_TOOLWINDOW),
    );

    let ctx = create_ctx_ex("Softui", window);
    let accent = accent_color();

    set_default_font_size(font_size);

    loop {
        //TODO: If the user didn't click in the window, close the program.
        // match wait_for_global_events() {
        //     Some(Event::Input(Key::LeftMouseDown, _)) => {}
        //     _ => {}
        // }

        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(0x202020.into());

        //Great layout code right here.
        let hp = Rect::new(0, 0, ctx.window.width(), rect_height);
        let b = hp.y(font_size + padding);
        let p = hp.y(2 * (font_size + padding));

        //Yeah this is pretty fast what can I say...?
        //TODO: The system takes a while to register the update.
        //Maybe just draw based on what the user clicks and not what windows does.
        match current_plan() {
            "High performance" => {
                ctx.draw_rectangle(hp.x, hp.y, hp.width, hp.height, accent);
            }
            "Balanced" => {
                ctx.draw_rectangle(b.x, b.y, b.width, b.height, accent);
            }
            "Power saver" => {
                ctx.draw_rectangle(p.x, p.y, p.width, p.height, accent);
            }
            _ => unreachable!(),
        }

        if ctx.window.left_mouse.clicked(hp) {
            high_performance();
        }

        if ctx.window.left_mouse.clicked(b) {
            balanced();
        }

        if ctx.window.left_mouse.clicked(p) {
            power_saver();
        }

        //This is the code I want.
        //How would I add the selection highlight?
        //I need `text().selected(|| {})` and  `text.hover(|| {})` functions
        // v!(
        //     text("High performance"),
        //     text("Balanced"),
        //     text("Power saver")
        // ).padding(padding);

        //TODO: Padding is not working.
        flex!(v!(text("High performance"), text("Balanced"), text("Power saver")).padding(padding));

        // ctx.draw_text(
        //     "High performance",
        //     default_font().unwrap(),
        //     hp.x + padding,
        //     hp.y,
        //     font_size,
        //     0,
        //     Color::WHITE,
        // );

        // ctx.draw_text(
        //     "Balanced",
        //     default_font().unwrap(),
        //     b.x + padding,
        //     b.y,
        //     font_size,
        //     0,
        //     Color::WHITE,
        // );

        // ctx.draw_text(
        //     "Power saver",
        //     default_font().unwrap(),
        //     p.x + padding,
        //     p.y,
        //     font_size,
        //     0,
        //     Color::WHITE,
        // );

        ctx.draw_frame();
    }
}
