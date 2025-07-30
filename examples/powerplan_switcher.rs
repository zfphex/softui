#![allow(unused)]
#[cfg(target_os = "windows")]
fn main() {
    use softui::*;
    use windows::*;

    let gap = 10;
    let font_size = 20;
    let rect_height = 30;

    let width = 360;
    let height = 3 * (font_size as i32 + gap as i32);

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
    ctx.set_fill_color(0x202020.into());

    set_default_font_size(font_size);

    let current_plan = std::cell::Cell::new(current_plan());

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

        //Great layout code right here.
        let hp = Rect::new(0, 0, ctx.window.width(), rect_height);
        let b = hp.y(font_size + gap);
        let p = hp.y(2 * (font_size + gap));

        //Yeah this is pretty fast what can I say...?
        //TODO: The system takes a while to register the update.
        //Maybe just draw based on what the user clicks and not what windows does.

        match current_plan.get() {
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

        //TODO: I want the flex to fill the viewport 100%
        //Then I want to fill the text 100% and change the background color.
        //Currently basic things like this will not work.
        flex!(
            text("High performance")
                //TODO: Text padding and margin.
                .w(p.width)
                // .bg(accent)
                //TODO: How to set rectangle border for this bad boy.
                // .bg((current_plan.get() == "High performance")
                //     .then_some(accent)
                //     .unwrap_or_default())
                .on_click(Left, |_| {
                    std::thread::spawn(|| high_performance());
                    current_plan.set("High performance");
                }),
            text("Balanced")
                .on_click(Left, |_| {
                    std::thread::spawn(|| balanced());
                    current_plan.set("Balanced");
                })
                .w(p.width),
            text("Power saver")
                .on_click(Left, |_| {
                    std::thread::spawn(|| power_saver());
                    current_plan.set("Power saver");
                })
                .w(p.width)
        )
        .direction(FlexDirection::TopBottom)
        .gap(gap);

        //TODO: Left pad
        // .left_pad(4);
        //TODO: How would I add the selection highlight?
        //I need `text().selected(|| {})` and  `text.hover(|| {})` functions

        ctx.draw_frame();
    }
}

#[cfg(target_os = "windows")]
mod windows {

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

    pub fn high_performance() {
        unsafe { PowerSetActiveScheme(core::ptr::null_mut(), &GUID_MIN_POWER_SAVINGS) };
    }

    pub fn balanced() {
        unsafe { PowerSetActiveScheme(core::ptr::null_mut(), &GUID_TYPICAL_POWER_SAVINGS) };
    }

    pub fn power_saver() {
        unsafe { PowerSetActiveScheme(core::ptr::null_mut(), &GUID_MAX_POWER_SAVINGS) };
    }

    pub fn current_plan() -> &'static str {
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

    pub fn accent_color() -> Color {
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
}

#[cfg(not(target_os = "windows"))]
fn main() {}
