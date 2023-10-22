use crate::prelude::Equation;
use crate::var::Variable;
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    symbols::line,
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

pub struct Graph<E: Equation> {
    pub equation: E,
    pub x_values: Vec<f64>,
}

impl<E: Equation> Graph<E> {
    pub fn new(equation: E, x_values: Vec<f64>) -> Self {
        Self { equation, x_values }
    }
}

impl<E: Equation> Widget for Graph<E> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let y_values: Vec<f64> = self
            .x_values
            .iter()
            .map(|&x| self.equation.eval(&Variable::new(x)))
            .collect();

        let max_y = *y_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let min_y = *y_values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let min_x = *self
            .x_values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let max_x = *self
            .x_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);

        for i in 1..self.x_values.len() {
            let (x1, y1) = to_pixel_coords(
                self.x_values[i - 1],
                y_values[i - 1],
                &area,
                min_x,
                max_x,
                min_y,
                max_y,
            );
            let (x2, y2) = to_pixel_coords(
                self.x_values[i],
                y_values[i],
                &area,
                min_x,
                max_x,
                min_y,
                max_y,
            );
            // bresenham_line(buf, x1 as f64, y1 as f64, x2 as f64, y2 as f64);
        }

        /*for (i, &y) in y_values.iter().enumerate() {
            let norm_y = normalize(y, min_y, max_y, area.height as f64);
            for row in 0..norm_y as u16 {
                buf.get_mut(area.x + i as u16, area.y + area.height - row - 1)
                    .set_symbol(line::VERTICAL)
                    .set_style(Style::default());
            }
        }*/

        draw_axes(buf, area, min_y, max_y);
        label_axes(buf, area, min_y, max_y);
        display_equation(buf, area, &self.equation);
    }
}

fn normalize(value: f64, min: f64, max: f64, height: f64) -> f64 {
    height * (value - min) / (max - min)
}

fn bresenham_line(buf: &mut Buffer, mut x1: f64, mut y1: f64, x2: f64, y2: f64) {
    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();

    let sx = if x1 < x2 { 1 } else { -1 } as f64;
    let sy = if y1 < y2 { 1 } else { -1 } as f64;

    let mut err = if dx > dy { dx } else { -dy } / 2f64;
    let mut err2;

    loop {
        if x1 >= 0f64 && x1 < buf.area().width as f64 && y1 >= 0f64 && y1 < buf.area().height as f64
        {
            let cell = buf.get_mut(x1 as u16, y1 as u16);
            cell.set_symbol("*").set_style(Style::default());
        }

        if x1 == x2 && y1 == y2 {
            break;
        }

        err2 = err;

        if err2 > -dx {
            err -= dy;
            x1 += sx;
        }
        if err2 < dy {
            err += dx;
            y1 += sy;
        }
    }
}

fn to_pixel_coords(
    x: f64,
    y: f64,
    area: &Rect,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
) -> (u16, u16) {
    let pixel_x_f64 = (x - min_x) / (max_x - min_x) * area.width as f64;
    let pixel_y_f64 = (y - min_y) / (max_y - min_y) * area.height as f64;

    let pixel_x = pixel_x_f64.clamp(0.0, u16::MAX as f64) as u16;
    let pixel_y = pixel_y_f64.clamp(0.0, u16::MAX as f64) as u16;

    let final_pixel_x = area.x.checked_add(pixel_x).unwrap_or(u16::MAX);
    let final_pixel_y = area
        .y
        .checked_add(u16::MAX.saturating_sub(pixel_y))
        .unwrap_or(0);

    (final_pixel_x, final_pixel_y)
}

fn draw_axes(buf: &mut Buffer, area: Rect, min_y: f64, max_y: f64) {
    for x in area.left()..area.right() {
        buf.get_mut(x, area.bottom() - 1)
            .set_symbol(line::HORIZONTAL)
            .set_style(Style::default());
    }

    for y in area.top()..area.bottom() {
        buf.get_mut(area.left(), y)
            .set_symbol(line::VERTICAL)
            .set_style(Style::default());
    }

    buf.get_mut(area.left(), area.bottom() - 1)
        .set_symbol(line::CROSS)
        .set_style(Style::default());
}

fn label_axes(buf: &mut Buffer, area: Rect, min_y: f64, max_y: f64) {
    let y_labels = vec![
        format!("{:.2}", max_y),
        format!("{:.2}", (max_y + min_y) / 2.0),
        format!("{:.2}", min_y),
    ];

    for (i, label) in y_labels.iter().enumerate() {
        buf.set_stringn(
            area.left().saturating_sub(label.len() as u16),
            area.top() + (area.height / 3 * i as u16),
            label,
            label.len(),
            Style::default(),
        );
    }
}

fn display_equation<E: Equation>(buf: &mut Buffer, area: Rect, equation: &E) {
    let equation_string = equation.to_string();
    let block = Block::default().title("Equation").borders(Borders::ALL);
    let rect = Rect {
        x: area.right() - equation_string.len() as u16 - 4,
        y: area.top(),
        width: equation_string.len() as u16 + 4,
        height: 3,
    };
    let para = Paragraph::new(Text::raw(equation_string)).block(block);
    Widget::render(para, rect, buf);
}
