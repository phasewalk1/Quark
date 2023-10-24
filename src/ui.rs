use crate::state::{App, Screen};
use ratatui::{
    prelude::{
        Alignment, Backend, Color, Constraint, Direction, Layout, Span, Style, Styled, Text,
    },
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    match app.screen {
        Screen::Home => {}
        Screen::Exiting => {}
        Screen::AddSample => {
            render_add_sample_screen(frame, app);
        }
        Screen::FileBrowser => {
            render_file_browser(frame, app);
        }
        Screen::KeyBinding => {}
    }
}

fn render_add_sample_screen<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let area = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let block = Block::default()
        .title("Load a sample")
        .borders(Borders::ALL);

    let styled_key = match app.load_sample_key {
        Some(k) => Span::styled(k.to_string(), Style::default().fg(Color::Green)),
        None => Span::raw("".to_string()),
    };

    let styled_path = match &app.load_sample_path {
        Some(path) => Span::styled(path.clone(), Style::default().fg(Color::Green)),
        None => Span::raw("".to_string()),
    };

    let line = Line::from(vec![
        Span::raw("Press key to bind: "),
        styled_key,
        Span::raw(" | Sample path: "),
        styled_path,
    ]);

    frame.render_widget(block, chunks[1]);
    frame.render_widget(
        Paragraph::new(vec![line])
            .block(Block::default())
            .alignment(Alignment::Center),
        chunks[1],
    );
    frame.render_widget(app.text_input.clone(), chunks[2]);
}

fn render_file_browser<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let area = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(area);

    let block = Block::default().title("Browser").borders(Borders::ALL);

    let curr_dir_line = Line::from(vec![Span::styled(
        app.file_browser.current_path.to_string_lossy(),
        Style::default().fg(Color::Yellow),
    )]);

    frame.render_widget(
        Paragraph::new(vec![curr_dir_line])
            .block(Block::default())
            .alignment(Alignment::Left),
        chunks[0],
    );

    let file_list: Vec<Line> = app
        .file_browser
        .entries
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let style = if i == app.file_browser.selected_index {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default()
            };
            Line::from(Span::styled(
                path.file_name().unwrap().to_string_lossy(),
                style,
            ))
        })
        .collect();

    frame.render_widget(
        Paragraph::new(file_list)
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left),
        chunks[1],
    );

    let instructions = Line::from(Span::styled(
        "< ^ >: Move | Enter: Traverse Into/Select | Tab: Traverse Back",
        Style::default().fg(Color::Green),
    ));

    frame.render_widget(
        Paragraph::new(vec![instructions])
            .block(Block::default())
            .alignment(Alignment::Left),
        chunks[2],
    );

    frame.render_widget(block, area);
}
