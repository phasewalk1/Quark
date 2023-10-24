use crossterm::event::KeyCode;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileBrowser {
    pub current_path: PathBuf,
    pub selected_index: usize,
    pub entries: Vec<PathBuf>,
}

impl Default for FileBrowser {
    fn default() -> Self {
        let current_path = std::env::current_dir().unwrap();
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

    pub fn selected_is_audio(&self) -> bool {
        let selected = self.selected_path().unwrap();
        match selected.extension() {
            Some(ext) => {
                return ext == "mp3" || ext == "wav" || ext == "ogg";
            }
            None => {
                return false;
            }
        }
    }

    pub fn selected_file(&self) -> String {
        self.selected_path()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
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
                if self.selected_is_audio() {
                    return;
                } else {
                    self.traverse_into(self.selected_index);
                    self.selected_index = 0;
                }
            }
            KeyCode::Tab => {
                self.backtrack(self.selected_index);
                self.selected_index = 0;
            }
            _ => {}
        }
    }
}
