use softui::*;

fn input_box<'a>(label: &'a str, todos: &'a mut Vec<Item>, input: &'a mut Option<String>) -> impl Widget<'a> + 'a {
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

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    let mut todos: Vec<Item> = Vec::new();
    let mut input: Option<String> = None;

    //0 = All, 1 = Active, 2 = Completed
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

        let todo_list: Vec<Text<'_>> = todos.iter().map(|i| text(i.label.clone())).collect();

        let list: Vec<Text<'_>> = match state {
            All => todos.iter().map(|i| text(i.label.clone())).collect(),
            Active => todos
                .iter()
                .filter(|i| !i.done)
                .map(|i| text(i.label.clone()))
                .collect(),
            Completed => todos
                .iter()
                .filter(|i| i.done)
                .map(|i| text(i.label.clone()))
                .collect(),
        };

        let root = v!(v!(
            //
            text("todos").size(22),
            input_box("What needs to be done?", &mut todos, &mut input),
            //This is really bad, why does fit work but v! doesn't
            fit!(
                v!(text(format!("{} task left", todo_list.len()))).w(50.percent()),
                //TODO: This sucks + the background doesn't cover the whole area.
                text("All")
                    .bg(if state == All { Some(cyan()) } else { None })
                    .on_click(Left, |_| state = All),
                text("Active")
                    .bg(if state == Active { Some(cyan()) } else { None })
                    .on_click(Left, |_| state = Active),
                text("Completed")
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
