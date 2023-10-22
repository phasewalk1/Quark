#![allow(dead_code, unused_variables)]

use crate::equation::LinearEquation;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use graph::Graph;
use prelude::Equation;
use ratatui::{
    prelude::{Backend, Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use state::{App, Screen};
use std::io::Error as IoError;
use var::Variable;

mod equation;
mod graph;
mod prelude;
mod state;
mod var;

fn main() -> Result<(), IoError> {
    enable_raw_mode()?;
    let mut stderr = std::io::stderr();

    crossterm::execute!(stderr, EnterAlternateScreen, LeaveAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app: App<LinearEquation> = state::App::default();
    let equation = LinearEquation::new(4.5f64, 2.0f64);
    app.set_equation(equation);

    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;

    terminal.show_cursor()?;

    Ok(())
}

pub fn run_app<B: Backend, E: Equation>(
    term: &mut Terminal<B>,
    app: &mut App<E>,
) -> std::io::Result<bool> {
    loop {
        term.draw(|frame| render_ui(frame, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => {
                    app.screen = Screen::Exiting;
                    return Ok(false);
                }
                _ => {}
            }
        }
    }
}

pub fn render_ui<B: Backend, E: Equation>(frame: &mut Frame<B>, app: &mut App<E>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled("Quark", Style::default().fg(Color::Magenta)))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    if let Some(equation) = &app.equation {
        let x_values: Vec<f64> = (-10..=10).map(|x| x as f64).collect();
        let graph = Graph::new(equation.clone(), x_values);

        frame.render_widget(graph, chunks[1]);
    }
}
