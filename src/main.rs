use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::{Error as IoError, Stderr};

mod sampler;
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

    let _ = event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;

    terminal.show_cursor()?;

    Ok(())
}

pub fn event_loop(
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
                    KeyCode::Enter => {
                        if app.file_browser.selected_is_audio() {
                            app.load_sample_path = Some(app.file_browser.selected_file());
                            app.screen = Screen::KeyBinding;
                        } else {
                            app.file_browser.handle_event(key.code);
                        }
                    }
                    _ => app.file_browser.handle_event(key.code),
                },
                Screen::KeyBinding => match key.code {
                    KeyCode::Char(c) => {
                        app.load_sample_key = Some(c);
                        app.sampler
                            .add_source(c, app.file_browser.selected_path().unwrap());
                    }
                    KeyCode::Enter => {
                        if app.load_sample_key.is_some() {
                            app.screen = Screen::Home;
                        }
                    }
                    _ => {}
                },
                Screen::AddSample => {}
                Screen::Home => match key.code {
                    KeyCode::Char('a') => {
                        app.screen = Screen::FileBrowser;
                    }
                    KeyCode::Char(c) => {
                        app.sampler.play_source(c);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
