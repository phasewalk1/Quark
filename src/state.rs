use crate::{
    sampler::QuickSampler,
    widgets::{browser::FileBrowser, input::TextInput},
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
    /// Sampler machine state
    pub sampler: QuickSampler,
    /// Path to a sample to load
    pub load_sample_path: Option<String>,
    /// Key to play a loaded sample
    pub load_sample_key: Option<char>,
    /// Text input state
    pub text_input: TextInput,
    /// File browser state
    pub file_browser: FileBrowser,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Home,
            sampler: QuickSampler::new(),
            load_sample_path: None,
            load_sample_key: None,
            text_input: TextInput::default(),
            file_browser: FileBrowser::default(),
        }
    }
}
