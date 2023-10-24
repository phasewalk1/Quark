use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Clone)]
pub struct Waveform {
    pub data: Vec<f32>,
    pub color: Color,
}

impl Waveform {
    pub fn new(data: Vec<f32>, color: Color) -> Self {
        Waveform { data, color }
    }
}

impl Widget for Waveform {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!();
    }
}
