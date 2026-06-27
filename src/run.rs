use crossterm::{
    event::{self, Event, KeyEvent},
    style::Stylize,
};
use ratatui::{
    DefaultTerminal,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{
        Color::{Blue, Green, Yellow},
        Style,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget},
};
use ratatui_textarea::TextArea;
use std::{fmt::format, io};

#[derive(PartialEq)]
pub enum State {
    Writing,
    Viewing,
    Editing,
}

pub struct Task {
    body: String,
    completed: bool,
    priority: i32,
}

impl Task {
    pub fn new(body: &str) -> Self {
        Self {
            body: body.to_string(),
            completed: false,
            priority: 0,
        }
    }
}

pub struct AppState {
    pub list_state: ListState,
    pub state: State,
    pub tasks: Vec<Task>,
    pub inputfield: TextArea<'static>,
    pub input_title: String,
}

impl AppState {
    fn selected(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }
}

pub fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> io::Result<()> {
    loop {
        // * ==============Update Variables===========
        // let mut selected = app.list_state.selected().unwrap_or(0);
        // * ==============Rendering===========
        terminal
            .draw(|f| {
                let mut chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        // Constraint::Length(3),
                    ])
                    .split(f.area());
                if app.state != State::Viewing {
                    chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(3),
                            // Constraint::Length(3),
                        ])
                        .split(f.area());
                }

                // * Title "TO-DO TUI"
                let title = Paragraph::new("TO-DO TUI")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    );

                f.render_widget(title, chunks[0]);

                // * To-do list & ListItem conversion
                let lower_title = Line::from(vec![
                    Span::styled(" Navigate ", Style::default()),
                    Span::styled("<Up/Down>", Style::default().fg(Blue).bold()),
                    Span::styled(" New Task ", Style::default()),
                    Span::styled("<A>", Style::default().fg(Blue).bold()),
                    Span::styled(" Edit Task ", Style::default()),
                    Span::styled("<E>", Style::default().fg(Blue).bold()),
                    Span::styled(" Cycle Priority ", Style::default()),
                    Span::styled("<R>", Style::default().fg(Blue).bold()),
                    Span::styled(" Mark Task ", Style::default()),
                    Span::styled("<Space>", Style::default().fg(Blue).bold()),
                    Span::styled(" Cancel/Exit ", Style::default()),
                    Span::styled("<ESC> ", Style::default().fg(Blue).bold()),
                ]);
                let list_block = Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("to-do")
                    .title_bottom(lower_title);

                let in_area = list_block.inner(chunks[1]);
                let available_width = in_area.width.saturating_sub(6) as usize;

                let items: Vec<ListItem> = app
                    .tasks
                    .iter()
                    .map(|task| {
                        let mut completion_lines = String::new();
                        if task.completed {
                            completion_lines = format!("C: {}", task.body.as_str());
                        } else {
                            completion_lines = format!("I: {}", task.body.as_str());
                        }
                        match task.priority {
                            0 => {
                                completion_lines = format!("! {}", completion_lines);
                            }
                            1 => {
                                completion_lines = format!("!! {}", completion_lines);
                            }
                            2 => {
                                completion_lines = format!("!!! {}", completion_lines);
                            }
                            _ => {}
                        }
                        let wrapped_lines = textwrap::wrap(&completion_lines, available_width);

                        let wrapped_text = wrapped_lines
                            .iter()
                            .map(|cow| cow.to_string())
                            .collect::<Vec<String>>()
                            .join("\n");
                        // if task.completed {
                        //     return ListItem::new(format!("I: {}", wrapped_text))
                        // }
                        // else {
                        //     return ListItem::new(format!("C: {}", wrapped_text))
                        // }
                        ListItem::new(wrapped_text)
                    })
                    .collect();

                let list = List::new(items)
                    .block(list_block)
                    .style(Style::new().white())
                    .highlight_style(Style::new().fg(Green))
                    .highlight_symbol("=> ")
                    .repeat_highlight_symbol(true)
                    .direction(ratatui::widgets::ListDirection::TopToBottom);

                f.render_stateful_widget(list, chunks[1], &mut app.list_state);

                // * Input field
                if app.state != State::Viewing {
                    let input_area = chunks[2];
                    let block = Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title(app.input_title.clone());
                    f.render_widget(&block, chunks[2]);
                    let inner_area = block.inner(chunks[2]);
                    f.render_widget(&app.inputfield, inner_area);
                }

                // * Exit clue
                // let footerprg = Paragraph::new("up/down to select, a to add, d to delete selected, esc to quit")
                //     .alignment(Alignment::Center)
                //     .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
                // let footerpos = chunks.len() - 1;
                // f.render_widget(footerprg, chunks[footerpos]);
            })
            .unwrap();

        // * ================Key Checks=====================
        if let Event::Key(key) = event::read()? {
            let exit = handle_key(app, key);
            if exit {
                break;
            }
        }
    }
    Ok(())
}

fn handle_key(app: &mut AppState, key: KeyEvent) -> bool {
    if app.state != State::Viewing {
        app.inputfield.input(key);
    }
    match key.code {
        // * Exit
        event::KeyCode::Esc => {
            if app.state == State::Viewing {
                return true;
            } else {
                app.state = State::Viewing;
            }
        }
        // * Select Down
        event::KeyCode::Down => {
            if app.selected() < app.tasks.len().saturating_sub(1) {
                app.list_state.select(Some(app.selected() + 1));
            } else if app.selected() == app.tasks.len().saturating_sub(1) {
                app.list_state.select(Some(0));
            }
        }
        // * Select Up
        event::KeyCode::Up => {
            if app.selected() > 0 && !app.tasks.is_empty() {
                app.list_state.select(Some(app.selected() - 1));
            } else if app.selected() == 0 && !app.tasks.is_empty() {
                app.list_state.select(Some(app.tasks.len() - 1));
            }
        }
        // * Delete task
        event::KeyCode::Char('d') => {
            if !app.tasks.is_empty() && app.state == State::Viewing {
                app.tasks.remove(app.selected());
            }
        }
        // * Enter writing mode
        event::KeyCode::Char('a') => {
            if app.state == State::Viewing {
                app.input_title = String::from("New Task");
                app.state = State::Writing;
                app.inputfield.clear();
            }
        }
        // * Edit task
        event::KeyCode::Char('e') => {
            if app.state == State::Viewing && !app.tasks.is_empty() {
                app.input_title = String::from("Edit Task");
                app.inputfield.clear();
                app.inputfield
                    .insert_str(app.tasks[app.selected()].body.to_string());
                app.state = State::Editing;
            }
        }
        // * Cycle priority
        event::KeyCode::Char('r') => {
            if !app.tasks.is_empty() {
                let list_selected = app.selected();
                let mut priority = app.tasks[list_selected].priority;
                priority += 1;
                priority %= 3;
                app.tasks[list_selected].priority = priority;
            }
        }
        // * Complete/Decomplete task
        event::KeyCode::Char(' ') => {
            if !app.tasks.is_empty() {
                app.tasks[app.list_state.selected().unwrap_or(0)].completed =
                    !app.tasks[app.selected()].completed;
            }
        }
        // * Submit new task
        event::KeyCode::Enter => {
            let selected = app.selected();
            if app.state == State::Writing && !app.inputfield.lines().join("").is_empty() {
                if app.tasks.is_empty() {
                    app.tasks
                        .insert(0, Task::new(&app.inputfield.lines().join("")));
                } else {
                    app.tasks
                        .insert(selected + 1, Task::new(&app.inputfield.lines().join("")));
                }
                app.state = State::Viewing;
            } else if app.state == State::Editing && !app.inputfield.lines().join("").is_empty() {
                app.tasks[selected].body = app.inputfield.lines().join("");
                app.state = State::Viewing;
            }
        }
        _ => {}
    }
    return false;
}

