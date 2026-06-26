#![allow(clippy::all)]
#![allow(unused)]
use crossterm::{event::{self, Event}, style::Stylize};
use ratatui::{
    DefaultTerminal, layout::{Alignment, Constraint, Direction, Layout}, style::{Color::{Blue, Green, Yellow}, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget}
};
use ratatui_textarea::TextArea;
use std::{fmt::format, io};
use textwrap::wrap;
mod run;

fn main() -> io::Result<()> {
    let mut app = run::AppState {
        list_state: ListState::default().with_selected(Some(0)),
        state: run::State::Viewing,
        tasks: vec![
            run::Task::new("Task 1"),
            run::Task::new("Task 2"),
            run::Task::new("Task 3")
        ],
        inputfield: TextArea::default(),
        input_title: String::new(),
    };
    let terminal = ratatui::init();
    let result = run::run(terminal, &mut app);
    ratatui::restore();
    result
}
