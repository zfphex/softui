#![allow(unused)]

use softui::*;

const GUID_MAX_POWER_SAVINGS: GUID = GUID::from_u128(0xa1841308_3541_4fab_bc81_f71556f20b4a);
const GUID_MIN_POWER_SAVINGS: GUID = GUID::from_u128(0x8c5e7fda_e8bf_4a96_9a85_a6e23a8c635c);
const GUID_TYPICAL_POWER_SAVINGS: GUID = GUID::from_u128(0x381b4222_f694_41f0_9685_ff5bb260df2e);

const WIDTH: i32 = 360;
const HEIGHT: i32 = 100;

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
        let mut policy = std::mem::zeroed();
        assert!(PowerGetActiveScheme(core::ptr::null_mut(), &mut policy) == 0);
        assert!(!policy.is_null());
        let p = &(*policy);

        match *p {
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
    let window = create_window(
        "PowerPlanSwitcher",
        1920 - WIDTH,
        1080 - HEIGHT,
        WIDTH,
        HEIGHT,
        WindowStyle::BORDERLESS.ex_style(WS_EX_TOPMOST | WS_EX_TOOLWINDOW),
    );
    let ctx = create_ctx_ex("Softui", window);

    let font_size = 20;
    let rect_height = 30;
    let padding = 10;
    let accent = accent_color();

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let power = current_plan();

        ctx.fill(0x202020.into());
        // ctx.draw_rectangle_scaled(
        //     0,
        //     0,
        //     ctx.window.width().unscaled(),
        //     rect_height,
        //     accent,
        //     0,
        //     Color::default(),
        //     0,
        // );

        ctx.draw_text(
            "High performance",
            default_font().unwrap(),
            0,
            0,
            font_size,
            0,
            Color::WHITE,
        );

        ctx.draw_rectangle(0, font_size + padding, ctx.window.width(), rect_height, accent);

        //TODO: This is not drawing at the correct y position.
        // ctx.draw_rectangle_scaled(
        //     0,
        //     font_size + padding,
        //     20,
        //     rect_height,
        //     accent,
        //     0,
        //     Color::default(),
        //     0,
        // );
        ctx.draw_text(
            "Balanced",
            default_font().unwrap(),
            0,
            font_size + padding,
            font_size,
            0,
            Color::WHITE,
        );

        // ctx.draw_rectangle(0, 2 * (font_size + padding), ctx.window.width(), rect_height, accent);
        // ctx.draw_rectangle_scaled(
        //     0,
        //     font_size + font_size + 2 * padding,
        //     ctx.window.width().unscaled(),
        //     rect_height,
        //     accent,
        //     0,
        //     Color::default(),
        //     0,
        // );
        ctx.draw_text(
            "Power saver",
            default_font().unwrap(),
            0,
            2 * (font_size + padding),
            font_size,
            0,
            Color::WHITE,
        );
        ctx.draw_frame();
    }
}
