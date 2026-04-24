use crate::core::world::WorldState;
use crate::render::compositor::{grid_to_lines, lines_to_grid};
use crate::render::figlet::render_figlet;
use crate::render::fonts::FontRegistry;
use crate::ui::state::UiState;
use crate::ui::widgets::clock::current_time_string;
use ratatui::{prelude::*, widgets::Paragraph};

fn render_lines(frame: &mut Frame, lines: &[String], start_x: u16, start_y: u16) {
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
    let lines = lines
        .iter()
        .map(|line| Line::from(line.clone()))
        .collect::<Vec<_>>();
    let grid = lines_to_grid(&lines, width, lines.len() as u16);
    let lines = grid_to_lines(&grid);
    for (i, line) in lines.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(line.clone()),
            Rect::new(start_x, start_y + i as u16, width, 1),
        );
    }
}

pub fn clock_lines(ui: &UiState, fonts: &FontRegistry) -> Vec<String> {
    let now = current_time_string();
    let font = fonts.get(ui.clock_font);
    render_figlet(font, &now)
}

#[allow(dead_code)]
pub fn draw_clock(
    frame: &mut Frame,
    _world: &WorldState,
    area: Rect,
    ui: &UiState,
    fonts: &FontRegistry,
) {
    let lines = clock_lines(ui, fonts);

    let height = lines.len() as u16;
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
    if width > area.width || height > area.height {
        return;
    }

    let x = area.x + (area.width - width) / 2;
    let y = area.y + (area.height - height) / 2;
    render_lines(frame, &lines, x, y);
}

#[allow(dead_code)]
pub fn draw_clock_at(
    frame: &mut Frame,
    _world: &WorldState,
    x: u16,
    y: u16,
    ui: &UiState,
    fonts: &FontRegistry,
) {
    let lines = clock_lines(ui, fonts);
    let height = lines.len() as u16;
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
    let start_x = x.saturating_sub(width / 2);
    let start_y = y.saturating_sub(height / 2);
    render_lines(frame, &lines, start_x, start_y);
}
