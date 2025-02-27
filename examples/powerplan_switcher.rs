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

    loop {
        //TODO: If the user didn't click in the window, close the program.
        // match wait_for_global_events() {
        //     Some(Event::Input(Key::LeftMouseDown, _)) => {}
        //     _ => {}
        // }

        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            //TODO: The event is not exposed by this api...
            //The context handles the clicked events.
            // Some(Event::Input(Key::LeftMouseDown, _)) => {
            //     clicked = true;
            // }
            // Some(Event::Input(Key::LeftMouseUp, _)) => {
            //     clicked = false;
            // }
            Some(event) => {
                dbg!(event);
            }
            _ => {}
        }

        let power = current_plan();

        ctx.fill(0x202020.into());

        //Yeah this is pretty fast what can I say...?

        let hp = Rect::new(0, 0, ctx.window.width(), rect_height);
        let b = Rect::new(0, font_size + padding, ctx.window.width(), rect_height);
        let p = Rect::new(0, 2 * (font_size + padding), ctx.window.width(), rect_height);

        //TODO: The system takes a while to register the update.
        //Maybe just draw based on what the user clicks and not what windows does.
        match power {
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

        if ctx.left_mouse.clicked(hp) {
            high_performance();
        }

        if ctx.left_mouse.clicked(b) {
            balanced();
        }

        if ctx.left_mouse.clicked(p) {
            power_saver();
        }

        ctx.draw_text(
            "High performance",
            default_font().unwrap(),
            hp.x,
            hp.y,
            font_size,
            0,
            Color::WHITE,
        );

        ctx.draw_text(
            "Balanced",
            default_font().unwrap(),
            b.x,
            b.y,
            font_size,
            0,
            Color::WHITE,
        );

        ctx.draw_text(
            "Power saver",
            default_font().unwrap(),
            p.x,
            p.y,
            font_size,
            0,
            Color::WHITE,
        );

        ctx.draw_frame();
    }
}
