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

fn input_box<'a>(label: &'a str, input: &'a mut Option<String>) -> impl Widget<'a> + 'a {
    let label = if let Some(input) = input { input.as_str() } else { label };
    v!(text(label).size(18))
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
    //Okay the whole fit!, v!, h! thing is a mess.
    fit!(
        v!().border(if item.done { None } else { Some(white()) })
            .wh(20)
            .bg(if item.done { Some(white()) } else { Some(black()) })
            //TODO: This does click repeats for some reason???
            .on_click(Left, |_| item.done = !item.done),
        //TODO: Spacers should work with bg(None)
        // rect().w(10.percent()).bg(black()),
        text(&item.label)
    )
    .gap(20)
    .hcenter()
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

        let len = todos.len();
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
            //
            text("todos").size(22),
            input_box("What needs to be done?", &mut input),
            //This is really bad, why does fit work but v! doesn't
            fit!(
                v!(text(format!("{} task left", len))).w(50.percent()),
                //TODO: This sucks + the background doesn't cover the whole area unless in a fit! container wtf?
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
            .gap(20)
        )
        .children(list)
        .gap(8)
        .pad(8)
        .hcenter());

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
