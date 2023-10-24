#![allow(dead_code, unused_variables, unused_imports)]

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Backend, Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle};
use std::{collections::HashMap, fs::File, path::Path};
use std::{
    io::{BufReader, Error as IoError, Stderr},
    path::PathBuf,
};

mod drums;
mod state;
mod ui;
mod widgets;

use state::{App, Screen};

fn main() -> Result<(), IoError> {
    enable_raw_mode()?;
    let mut stderr = std::io::stderr();

    crossterm::execute!(stderr, EnterAlternateScreen, LeaveAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app: App = App::default();

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

pub fn run_app(
    term: &mut Terminal<CrosstermBackend<Stderr>>,
    app: &mut App,
) -> std::io::Result<bool> {
    loop {
        term.draw(|frame| ui::render_ui(frame, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            if key.code == KeyCode::Char('q') {
                app.screen = Screen::Exiting;
                return Ok(false);
            }
            match app.screen {
                Screen::FileBrowser => match key.code {
                    KeyCode::Up => app.file_browser.up(),
                    KeyCode::Down => app.file_browser.down(),
                    _ => app.file_browser.handle_event(key.code),
                },
                Screen::KeyBinding => {}
                Screen::AddSample => {}
                Screen::Home => match key.code {
                    KeyCode::Char('a') => {
                        app.screen = Screen::FileBrowser;
                    }
                    _ => {}
                },
                _ => {}
            }
            /*match key.code {
                KeyCode::Char('q') => {
                    app.screen = Screen::Exiting;
                    return Ok(false);
                }
                KeyCode::Char('a') => {
                    app.screen = Screen::FileBrowser;
                }
                KeyCode::Down => match app.screen {
                    Screen::FileBrowser => {
                        app.file_browser.handle_event(KeyCode::Down);
                    }
                    _ => {}
                },
                KeyCode::Char(c) => match app.screen {
                    Screen::Home => {
                        app.drum_machine.play_source(c);
                    }
                    Screen::FileBrowser => {
                        app.file_browser.handle_event(key.code);
                        if let Some(selected_path) = app.file_browser.selected_path() {
                            app.selected_sample_path = Some(selected_path);
                            app.screen = Screen::KeyBinding;
                        }
                    }
                    Screen::KeyBinding => {}
                    Screen::AddSample => {
                        if let Some(_) = app.load_sample_key {
                            app.text_input.handle_event(key.code);
                            app.load_sample_path = Some(app.text_input.content.clone());
                        } else {
                            app.load_sample_key = Some(c);
                        }
                    }
                    _ => {
                        app.drum_machine.play_source(c);
                    }
                },
                _ => {}
            }*/
        }
    }
}
