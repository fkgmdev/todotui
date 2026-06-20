#![allow(clippy::all)]
#![allow(unused)]
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, layout::{Alignment, Constraint, Direction, Layout}, style::{Color::{Green, Yellow}, Style}, widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph}
};
use std::io;

enum State {
    Writing,
    Viewing,
    Editing,
}

struct AppState {
    list_state: ListState,
    state: State,
    tasks: Vec<String>,
}

fn main() -> io::Result<()> {
    let mut app = AppState {
        list_state: ListState::default().with_selected(Some(0)),
        state: State::Viewing,
        tasks: vec!["Task 1".to_string(), "Task 2".to_string(), "Task 3".to_string()],
    };
    let terminal = ratatui::init();
    let result = run(terminal, &mut app);
    ratatui::restore();
    result
}
fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> io::Result<()> {
    loop {
        // ==============Update Variables===========
        let selected = app.list_state.selected().unwrap_or(0);
        // ==============Rendering===========
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(f.area());

                // Title "TO-DO TUI"
                let title = Paragraph::new("TO-DO TUI")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));

                f.render_widget(title, chunks[0]);

                // To-do list & ListItem conversion
                let items: Vec<ListItem> = app
                    .tasks
                    .iter()
                    .map(|s| ListItem::new(s.as_str()))
                    .collect();

                let list = List::new(items)
                        .block(Block::bordered().border_type(BorderType::Rounded).title("to-do"))
                        .style(Style::new().white())
                        .highlight_style(Style::new().fg(Green))
                        .highlight_symbol("=>")
                        .repeat_highlight_symbol(true)
                        .direction(ratatui::widgets::ListDirection::TopToBottom);

                f.render_stateful_widget(list, chunks[1], &mut app.list_state);
                
                // Exit clue
                let footerprg = Paragraph::new("ESC to exit")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));

                f.render_widget(footerprg, chunks[2]);
            })
            .unwrap();

        // ================Key Checks=====================
        if let Event::Key(key) = event::read()? {
            match key.code {
                // Exit
                event::KeyCode::Esc => {
                    break;
                }
                // Select Down
                event::KeyCode::Down => {
                    if selected < (app.tasks.len() - 1) {
                        app.list_state.select(Some(selected + 1));
                    }
                    else if selected == (app.tasks.len() - 1) {
                        app.list_state.select(Some(0));
                    }
                }
                // Select Up
                event::KeyCode::Up => {
                    if selected > 0 && app.tasks.is_empty() == false {
                        app.list_state.select(Some(selected - 1));
                    }
                    else if selected == 0 && app.tasks.is_empty() == false {
                        app.list_state.select(Some(app.tasks.len() - 1));
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
