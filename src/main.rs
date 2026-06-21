#![allow(clippy::all)]
#![allow(unused)]
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, layout::{Alignment, Constraint, Direction, Layout}, style::{Color::{Green, Yellow}, Style}, widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget}
};
use ratatui_textarea::TextArea;
use std::io;

#[derive(PartialEq)]
enum State {
    Writing,
    Viewing,
    Editing,
}

struct Task {
    body: String,
    completed: bool,
}

impl Task {
    fn new(body: &str) -> Self {
        Self {
            body: body.to_string(),
            completed: false,
        }
    }
}
struct AppState {
    list_state: ListState,
    state: State,
    tasks: Vec<Task>,
    inputfield: TextArea<'static>,
}

fn main() -> io::Result<()> {
    let mut app = AppState {
        list_state: ListState::default().with_selected(Some(0)),
        state: State::Viewing,
        tasks: vec![
            Task::new("Task 1"),
            Task::new("Task 2"),
            Task::new("Task 3")
        ],
        inputfield: TextArea::default(),
    };
    let terminal = ratatui::init();
    let result = run(terminal, &mut app);
    ratatui::restore();
    result
}
fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> io::Result<()> {
    loop {
        // * ==============Update Variables===========
        let mut selected = app.list_state.selected().unwrap_or(0);
        // * ==============Rendering===========
        terminal
            .draw(|f| {
                let mut chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(f.area());
                if app.state != State::Viewing {
                    chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ])
                    .split(f.area());
                }

                // * Title "TO-DO TUI"
                let title = Paragraph::new("TO-DO TUI")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));

                f.render_widget(title, chunks[0]);

                // * To-do list & ListItem conversion
                let items: Vec<ListItem> = app
                    .tasks
                    .iter()
                    .map(|task| ListItem::new(task.body.as_str()))
                    .collect();

                let list = List::new(items)
                        .block(Block::bordered().border_type(BorderType::Rounded).title("to-do"))
                        .style(Style::new().white())
                        .highlight_style(Style::new().fg(Green))
                        .highlight_symbol("=>")
                        .repeat_highlight_symbol(true)
                        .direction(ratatui::widgets::ListDirection::TopToBottom);

                f.render_stateful_widget(list, chunks[1], &mut app.list_state);

                // * Input field
                if app.state != State::Viewing {
                    let input_area = chunks[2];
                    let block = Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title("New Task");
                    f.render_widget(block.clone(), chunks[2]);
                    let inner_area = block.inner(chunks[2]);
                    f.render_widget(&app.inputfield, inner_area);
                }
                
                // * Exit clue
                let footerprg = Paragraph::new("ESC to exit")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
                let footerpos = chunks.len() - 1;
                f.render_widget(footerprg, chunks[footerpos]);
            })
            .unwrap();

        // * ================Key Checks=====================
        if let Event::Key(key) = event::read()? {
            if app.state != State::Viewing {
                app.inputfield.input(key);
            }
            match key.code {
                // * Exit
                event::KeyCode::Esc => {
                    if app.state == State::Viewing {
                        break;
                    }
                    else {
                        app.state = State::Viewing;
                    }
                }
                // * Select Down
                event::KeyCode::Down => {
                    if selected < app.tasks.len().saturating_sub(1) {
                        app.list_state.select(Some(selected + 1));
                    }
                    else if selected == app.tasks.len().saturating_sub(1) {
                        app.list_state.select(Some(0));
                    }
                }
                // * Select Up
                event::KeyCode::Up => {
                    if selected > 0 && app.tasks.is_empty() == false {
                        app.list_state.select(Some(selected - 1));
                    }
                    else if selected == 0 && app.tasks.is_empty() == false {
                        app.list_state.select(Some(app.tasks.len() - 1));
                    }
                }
                // * Delete task
                event::KeyCode::Char('d') => {
                    if app.tasks.is_empty() == false && app.state == State::Viewing {
                        app.tasks.remove(app.list_state.selected().unwrap_or(0));
                    }
                }
                // * Enter writing mode
                event::KeyCode::Char('a') => {
                    if app.state == State::Viewing {
                        app.state = State::Writing;
                        app.inputfield.clear();
                    }
                }
                // * Submit new task
                event::KeyCode::Enter => {
                    if app.state == State::Writing && app.inputfield.lines().join("").is_empty() == false {
                        selected = app.list_state.selected().unwrap_or(0);
                        if app.tasks.is_empty() {
                            app.tasks.insert(0, Task::new(&app.inputfield.lines().join("")));
                        }
                        else {
                            app.tasks.insert(selected + 1, Task::new(&app.inputfield.lines().join("")));
                        }
                        app.state = State::Viewing;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
