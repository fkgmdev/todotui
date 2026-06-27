#![allow(clippy::all)]
#![allow(unused)]
use ratatui::widgets::ListState;
use ratatui_textarea::TextArea;
use std::io;
mod run;

fn main() -> io::Result<()> {
    let mut app = run::AppState {
        list_state: ListState::default().with_selected(Some(0)),
        state: run::State::Viewing,
        // tasks: vec![
        //     run::Task::new("Task 1"),
        //     run::Task::new("Task 2"),
        //     run::Task::new("Task 3")
        // ],
        tasks: run::load_list().unwrap_or(vec![
            run::Task::new("Couldn't load list"),
            run::Task::new("This is the default list"),
            run::Task::new("Task 3")
        ]),
        inputfield: TextArea::default(),
        input_title: String::new(),
    };
    let terminal = ratatui::init();
    let result = run::run(terminal, &mut app);
    ratatui::restore();
    result
}
