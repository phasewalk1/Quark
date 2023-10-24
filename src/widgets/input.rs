use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Clone)]
pub struct TextInput {
    pub content: String,
    pub cursor: usize,
    pub focused: bool,
}

impl Default for TextInput {
    fn default() -> Self {
        TextInput {
            content: String::new(),
            cursor: 0,
            focused: false,
        }
    }
}

impl TextInput {
    pub fn handle_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.content.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.content.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            KeyCode::Delete => {
                if self.cursor < self.content.len() {
                    self.content.remove(self.cursor);
                }
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor < self.content.len() {
                    self.cursor += 1;
                }
            }
            _ => {}
        }
    }
}

impl Widget for TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cursor_symbol = if self.focused { "_" } else { " " };
        let rendered_text = format!("{}{}", &self.content[..self.cursor], cursor_symbol);
        let text_span = Span::styled(rendered_text, Style::default().fg(Color::White));
        let line = Line::from(vec![text_span]);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
