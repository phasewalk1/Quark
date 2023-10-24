use std::{io::Stderr, path::PathBuf};

use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    prelude::{Backend, Buffer, CrosstermBackend, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame,
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

// impl widget trait
impl Widget for TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cursor_symbol = if self.focused { "_" } else { " " };
        let rendered_text = format!("{}{}", &self.content[..self.cursor], cursor_symbol);
        let text_span = Span::styled(rendered_text, Style::default().fg(Color::White));
        let line = Line::from(vec![text_span]);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}

#[derive(Clone)]
pub struct FileBrowser {
    pub current_path: PathBuf,
    pub selected_index: usize,
    pub entries: Vec<PathBuf>,
}

impl Default for FileBrowser {
    fn default() -> Self {
        let current_path = std::env::current_dir().unwrap();
        // get all file entries in current path
        let entries: Vec<PathBuf> = current_path
            .read_dir()
            .unwrap()
            .map(|r| r.unwrap().path())
            .collect();

        FileBrowser {
            current_path,
            selected_index: 0,
            entries,
        }
    }
}

impl FileBrowser {
    pub fn up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn traverse_into(&mut self, index: usize) {
        let entry = &self.entries[index];
        if entry.is_dir() {
            self.current_path = entry.clone();
            self.entries = self
                .current_path
                .read_dir()
                .unwrap()
                .map(|r| r.unwrap().path())
                .collect();
        }
    }

    pub fn backtrack(&mut self, index: usize) {
        if index == 0 {
            self.current_path = self.current_path.parent().unwrap().to_path_buf();
            self.entries = self
                .current_path
                .read_dir()
                .unwrap()
                .map(|r| r.unwrap().path())
                .collect();
        }
    }

    pub fn selected_path(&self) -> Option<PathBuf> {
        self.entries.get(self.selected_index).cloned()
    }

    pub fn handle_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_index < self.entries.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Enter => {
                self.traverse_into(self.selected_index);
                self.selected_index = 0;
            }
            KeyCode::Tab => {
                self.backtrack(self.selected_index);
                self.selected_index = 0;
            }
            _ => {}
        }
    }
}
