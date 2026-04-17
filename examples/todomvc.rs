// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! Notes
//! - Poor input management
//! - Confusing layout rules
//! - Lifetime and borrowing issues using closures
use softui::*;
use std::cell::Cell;
use State::*;

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    All,
    Active,
    Completed,
}

#[derive(Debug)]
pub struct Item {
    pub label: String,
    pub done: bool,
    pub editing: bool,
}

fn input_box<'a>(input: &'a Cell<Option<String>>) -> impl Widget<'a> + 'a {
    let label = if let Some(input) = unsafe { &*input.as_ptr() } {
        text(input.as_str())
    } else {
        text("What needs to be done?").fg(Some(gray()))
    };

    v!(label.size(18))
        .w(50.percent())
        .h(64)
        .bg(Some(black()))
        .border(Some(white()))
        .center()
        .on_lose_focus(|_| input.set(None))
        // .on_key_press(|key, _| {
        //     if let Some(input) = unsafe { &mut *input.as_ptr() } {
        //         input.push_str(key.as_str());
        //     }
        // })
        .on_click(Left, |_| input.set(Some(String::new())))

    //Push the todo that user typed.
    // .on_key_press(Key::Enter, |_| {
    //     todos.push("test");
    // })
}

//80% Item Name, 20% Edit Icon
fn item<'a>(item: &'a mut Item, input: &'a Cell<Option<String>>, pencil: &Svg) -> impl Widget<'a> + 'a {
    let checkbox = v!()
        .border(if item.done { None } else { Some(white()) })
        .wh(20)
        .bg(if item.done { Some(white()) } else { Some(black()) })
        .on_click(Left, |_| item.done = !item.done);

    if item.editing {
        fit!(
            v!().wh(20),
            text("_"),
            svg_ref(&pencil).on_click(Left, |_| item.editing = true)
        )
        .gap(10)
    } else {
        let pen = svg_ref(&pencil)
            // .on_lose_focus(|_| item.editing = false),
            .on_click(Left, |_| {
                item.editing = true;
                input.replace(Some(String::new()));
            });

        fit!(checkbox, text(&item.label), pen).gap(10)
    }
}

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    let mut todos: Vec<Item> = vec![
        Item {
            label: "Do the shopping.".into(),
            done: false,
            editing: false,
        },
        Item {
            label: "Walk the dog.".into(),
            done: false,
            editing: false,
        },
        Item {
            label: "Ponder existence...".into(),
            done: false,
            editing: false,
        },
    ];
    // let mut todos: Vec<Item> = Vec::new();

    let pencil = svg(include_bytes!("../img/pencil.svg"), 0.8, true);

    //The ergonomics of input here are impossible to use.
    let mut input: Cell<Option<String>> = Cell::new(None);
    let state: Cell<State> = Cell::new(All);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(Key::Backspace, m)) => {
                if let Some(input) = input.get_mut() {
                    if !input.trim().is_empty() {
                        //Modifiers don't work on macos.
                        if m.control {
                            input.clear();
                        } else {
                            input.pop();
                        };
                    }
                }
            }
            Some(Event::Input(Key::Enter, _)) => {
                if input.get_mut().is_some() {
                    todos.push(Item {
                        label: input.take().unwrap(),
                        done: false,
                        editing: false,
                    });
                }
            }
            Some(Event::Input(Key::Space, _)) => {
                if let Some(input) = input.get_mut() {
                    input.push(' ');
                }
            }
            Some(Event::Input(Key::Char(ch), _)) => {
                if let Some(input) = input.get_mut() {
                    input.push(ch);
                }
            }
            _ => {}
        }

        let remaining = todos.iter().filter(|t| !t.done).count();

        let list: Vec<_> = todos
            .iter_mut()
            .filter(|i| match state.get() {
                All => true,
                Active => !i.done,
                Completed => i.done,
            })
            .map(|i| item(i, &input, &pencil))
            .collect();

        let s = state.get();
        let sr = &state;
        let tab = |label: &'static str, target: State| {
            text(label)
                .bg(if s == target { Some(hex("3232B0")) } else { None })
                .p(2)
                .radius(6)
                .on_click(Left, move |_| sr.set(target))
        };

        let root = v!(
            text("todos").size(48).pb(12),
            input_box(&input),
            fit!(
                text(format!("{} task left", remaining)),
                tab("All", All),
                tab("Active", Active),
                tab("Completed", Completed)
            )
            .gap(20),
            if list.is_empty() {
                v!(text("You have not created a task yet...").fg(gray()))
                    .h(25.percent())
                    .vcenter()
                    .hcenter()
                //TODO: Percentage padding does not work yet.
                // .p(20.percent())
            } else {
                //Note: this fills the horizontal space (is this something we want?)
                v!().children(list).w(50.percent()).gap(8)
            }
        )
        .p(8)
        .gap(8)
        // .vcenter()
        .hcenter();

        ctx.draw_layout(root, true);
        ctx.draw_frame();
    }
}
