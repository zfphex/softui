use softui::*;

fn input_box<'a>(label: &'a str, todos: &'a mut Vec<String>, input: &'a mut Option<String>) -> impl Widget<'a> + 'a {
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

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    let mut todos: Vec<String> = Vec::new();
    let mut input: Option<String> = None;

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
                    todos.push(input.take().unwrap());
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

        let todo_list: Vec<Text<'_>> = todos.iter().map(|i| text(i.clone())).collect();

        let root = v!(
            v!(
                //
                text("todos").size(22),
                input_box("What needs to be done?", &mut todos, &mut input),
            )
            .children(todo_list)
            .gap(8)
            .pad(8)
            .hcenter(),
            // input("What needs to be done?"),
            // h!(text("Tasks left"), button("All"), button("Active"), button("Completed"))
        );
        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
