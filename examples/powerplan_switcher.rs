#![allow(unused)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
fn main() {
    use std::cell::Cell;

    use softui::*;
    use windows::*;

    const WIDTH: i32 = 360;
    const HEIGHT: i32 = 141;
    const TASK_BAR_HEIGHT: i32 = 40;
    const HP: &str = "High performance";
    const BL: &str = "Balanced";
    const PS: &str = "Power saver";
    const BACKGROUND: Color = hex("#1c1c1c");
    // const BACKGROUND: Color = Color::from(0x202020);
    const PADDING: usize = 13;

    let mut window = create_window(
        "Power Plan Switcher",
        1920 - WIDTH,
        1080 - HEIGHT - TASK_BAR_HEIGHT,
        WIDTH,
        HEIGHT,
        WindowStyle::BORDERLESS.ex_style(WS_EX_TOPMOST | WS_EX_TOOLWINDOW),
    );

    let rect = get_window_rect(window.hwnd);
    let width = (rect.right - rect.left) as usize;
    let height = (rect.bottom - rect.top) as usize;
    let area = Rect {
        x: rect.left as usize,
        y: rect.top as usize,
        width,
        height,
    };

    windows::create_tray_icon(window.hwnd);

    let mut ctx = Context::new(window);
    ctx.set_fill_color(BACKGROUND);

    //TODO: What is this cringe.
    set_default_font_size(16);

    let accent = Color::from(accent_color());

    let hover = hex("#423c4a");
    //TODO: This looks mid, older version looked better.
    // let hover = accent.adjust(0.5);

    //TODO: Close after a delay.
    // let end = false;
    let mode = Cell::new(current_plan());

    let mut draw = false;

    unsafe { watch_global_mouse_events() };

    loop {
        let lm = &mut global_state().left_mouse;

        //TODO: This should not trigger if the tray icon was just pressed. Maybe add a small delay idk?
        if ctx.window.tray.is_pressed() {
            draw = true;
        } else if lm.clicked() {
            if let Some(pos) = &lm.release_position {
                let pos = Rect::new(pos.get_x() as usize, pos.get_y() as usize, 1, 1);
                //User clicked outside the window.
                if !area.intersects(pos) {
                    dbg!(pos);
                    draw = false;
                }
            }
        }

        //TODO: Check if the user clicked outside of the window bounds and then hide the window.
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        fn item<'a>(
            plan: &'static str,
            accent: Color,
            hover: Color,
            mode: &'a Cell<&'static str>,
            selected: bool,
            change_plan: fn() -> (),
        ) -> Container<'a> {
            let m = mode;
            let change_plan = change_plan;
            fit!(text(plan).w(WIDTH))
                .on_click(Left, move |_| {
                    std::thread::spawn(change_plan);
                    m.set(plan);
                })
                //TODO: The cursors positions is not updated when leaving the window.
                //This will cause items to be falsely hovered.
                .on_hover(move |fit| {
                    if !selected {
                        fit.style.background_color = Some(hover);
                    }
                })
                .bg(if selected { Some(accent) } else { None })
                .pad(PADDING)
        }

        if draw {
            let root = v!(
                item(HP, accent, hover, &mode, mode.get() == HP, high_performance),
                item(BL, accent, hover, &mode, mode.get() == BL, balanced),
                item(PS, accent, hover, &mode, mode.get() == PS, power_saver)
            )
            .bg(BACKGROUND);
            ctx.draw_layout(root);
            ctx.debug_layout();
            ctx.draw_frame();
        } else {
            ctx.draw_frame();
        }
        // else if !ctx.window.hidden {

        //     ctx.window.hide();
        // }
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

    pub fn create_tray_icon(hwnd: isize) {
        unsafe {
            let h_icon = LoadIconA(core::ptr::null_mut(), IDI_APPLICATION as *const i8);
            window::create_tray_icon(hwnd, 1, h_icon, "Power Plan Switcher");
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
