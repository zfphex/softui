use softui::*;
use State::*;

#[derive(PartialEq)]
pub enum State {
    All,
    Active,
    Completed,
}

pub struct Item {
    pub label: String,
    pub done: bool,
}

fn input_box<'a>(input: &'a mut Option<String>) -> impl Widget<'a> + 'a {
    let label = if let Some(input) = input {
        text(input.as_str())
    } else {
        text("What needs to be done?").fg(gray())
    };

    v!(label.font_size(18))
        .w(50.percent())
        .h(64)
        .bg(black())
        .border(white())
        .center()
        .on_click(Left, |_| {
            *input = Some(String::new());
            //On click focus the input box and allow the user to type.
        })
    //Push the todo that user typed.
    // .on_key_press(Key::Enter, |_| {
    //     todos.push("test");
    // })
}

fn item<'a>(item: &'a mut Item) -> impl Widget<'a> + 'a {
    fit!(
        v!().border(if item.done { None } else { Some(white()) })
            .wh(20)
            .bg(if item.done { Some(white()) } else { Some(black()) })
            .on_click(Left, |_| item.done = !item.done),
        text(&item.label)
    )
    .vcenter()
    .gap(10)
}

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    let mut todos: Vec<Item> = vec![Item {
        label: "Test".into(),
        done: false,
    }];
    let mut input: Option<String> = None;
    let mut state = All;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(Key::Backspace, m)) => {
                if let Some(ref mut input) = input {
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
                if input.is_some() {
                    todos.push(Item {
                        label: input.take().unwrap(),
                        done: false,
                    });
                }
            }
            Some(Event::Input(Key::Space, _)) => {
                if let Some(ref mut input) = input {
                    input.push(' ');
                }
            }
            Some(Event::Input(Key::Char(ch), _)) => {
                if let Some(ref mut input) = input {
                    input.push(ch);
                }
            }
            _ => {}
        }

        let remaining = todos.iter().filter(|t| !t.done).count();
        let list: Vec<_> = todos
            .iter_mut()
            .filter(|i| match state {
                All => true,
                Active => !i.done,
                Completed => i.done,
            })
            .map(|i| item(i))
            .collect();

        let root = v!(v!(
            text("todos").font_size(22),
            input_box(&mut input),
            fit!(
                v!(text(format!("{} task left", remaining))).w(50.percent()),
                fit!(text("All"))
                    .bg(if state == All { Some(cyan()) } else { None })
                    .on_click(Left, |_| state = All),
                fit!(text("Active"))
                    .bg(if state == Active { Some(cyan()) } else { None })
                    .on_click(Left, |_| state = Active),
                fit!(text("Completed"))
                    .bg(if state == Completed { Some(cyan()) } else { None })
                    .on_click(Left, |_| state = Completed),
            )
            .gap(20),
            v!().children(list).w(50.percent()).gap(8)
        )
        .gap(8)
        .p(8)
        .hcenter());

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
