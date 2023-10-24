use std::path::PathBuf;

use crate::{
    drums::DrumMachine,
    widgets::{FileBrowser, TextInput},
};

/// Active screen
pub enum Screen {
    Home,
    AddSample,
    FileBrowser,
    KeyBinding,
    Exiting,
}

/// App state
pub struct App {
    /// Active screen
    pub screen: Screen,
    /// Drum Machine
    pub drum_machine: DrumMachine,
    /// Path to a sample to load
    pub load_sample_path: Option<String>,
    /// Key to play a loaded sample
    pub load_sample_key: Option<char>,
    /// Text input
    pub text_input: TextInput,
    pub file_browser: FileBrowser,
    pub selected_sample_path: Option<PathBuf>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Home,
            drum_machine: DrumMachine::new(),
            load_sample_path: None,
            load_sample_key: None,
            text_input: TextInput::default(),
            file_browser: FileBrowser::default(),
            selected_sample_path: None,
        }
    }
}
