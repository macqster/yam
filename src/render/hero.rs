use crate::ui::viewport::Viewport;
use ratatui::{
    prelude::*,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::sync::mpsc::Receiver;

pub struct Hero {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
    pub frames: Vec<Vec<Line<'static>>>,
    pub current_frame: usize,
    pub playing: bool,
    pub rx: Receiver<Vec<Line<'static>>>,
    step_once: bool,
}

impl Hero {
    pub fn new(world_width: usize, world_height: usize) -> Self {
        let rx = crate::render::chafa::hero_stream(96, 48);
        let frame = crate::render::chafa::hero_stream_initial_frame(96, 48);
        let frame = if frame.is_empty() {
            vec![Line::from("chafa unavailable")]
        } else {
            frame
        };

        let width = frame.iter().map(Line::width).max().unwrap_or(0) as u16;
        let height = frame.len() as u16;

        Self {
            x: (world_width / 2) as i32,
            y: (world_height / 2) as i32,
            width,
            height,
            frames: vec![frame],
            current_frame: 0,
            playing: true,
            rx,
            step_once: false,
        }
    }

    pub fn frame(&self) -> &Vec<Line<'static>> {
        self.frames
            .get(self.current_frame)
            .or_else(|| self.frames.first())
            .expect("hero always has at least one frame")
    }

    pub fn tick(&mut self) {
        while let Ok(frame) = self.rx.try_recv() {
            self.height = frame.len() as u16;
            self.width = frame.iter().map(Line::width).max().unwrap_or(0) as u16;
            self.frames.push(frame);
            if self.frames.len() > 128 {
                self.frames.remove(0);
                if self.current_frame > 0 {
                    self.current_frame -= 1;
                }
            }
        }

        if self.frames.is_empty() {
            return;
        }

        if self.playing {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        } else {
            if self.step_once {
                if self.current_frame + 1 < self.frames.len() {
                    self.current_frame += 1;
                }
                self.step_once = false;
            }
        }
    }

    pub fn toggle_animation(&mut self) {
        self.playing = !self.playing;
        self.step_once = false;
    }

    pub fn step_animation(&mut self) {
        if !self.playing {
            self.step_once = true;
        }
    }

    pub fn debug_rect(&self) -> (i32, i32, u16, u16) {
        let x = self.x - (self.width as i32 / 2);
        let y = self.y - (self.height as i32 / 2);
        (x, y, self.width, self.height)
    }
}

pub fn draw_hero(
    frame: &mut Frame,
    hero: &Hero,
    viewport: &Viewport,
    offset_x: i32,
    offset_y: i32,
) {
    let screen_x = hero.x - viewport.x;
    let screen_y = hero.y - viewport.y;
    let start_x = screen_x - (hero.width as i32 / 2) + offset_x;
    let start_y = screen_y - (hero.height as i32 / 2) + offset_y;
    let area = frame.area();

    if start_x >= area.right() as i32
        || start_y >= area.bottom() as i32
        || start_x + hero.width as i32 <= area.x as i32
        || start_y + hero.height as i32 <= area.y as i32
    {
        return;
    }

    let skip_cols = (area.x as i32 - start_x).max(0) as usize;
    let skip_rows = (area.y as i32 - start_y).max(0) as usize;
    let start_x = start_x.max(area.x as i32) as u16;
    let start_y = start_y.max(area.y as i32) as u16;
    render_lines_clipped(frame, hero.frame(), start_x, start_y, skip_cols, skip_rows);
}

fn render_lines_clipped(
    frame: &mut Frame,
    lines: &[Line<'static>],
    start_x: u16,
    start_y: u16,
    skip_cols: usize,
    skip_rows: usize,
) {
    let width = lines.iter().map(Line::width).max().unwrap_or(0) as u16;
    for (i, line) in lines.iter().skip(skip_rows).enumerate() {
        let text = line
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();
        let clipped = text.chars().skip(skip_cols).collect::<String>();
        if clipped.is_empty() {
            continue;
        }
        frame.render_widget(
            Paragraph::new(clipped),
            Rect::new(
                start_x,
                start_y + i as u16,
                width.saturating_sub(skip_cols as u16),
                1,
            ),
        );
    }
}

pub fn draw_hero_debug(
    frame: &mut Frame,
    hero: &Hero,
    viewport: &Viewport,
    offset_x: i32,
    offset_y: i32,
) {
    let (hx, hy, hw, hh) = hero.debug_rect();
    let title = format!(" hero box, x={:+}, y={:+} ", offset_x, offset_y);

    let mut visible_left = None;
    let mut visible_top = None;
    let mut visible_right = None;
    let mut visible_bottom = None;

    for dx in 0..hw {
        for dy in 0..hh {
            let is_border = dx == 0 || dx == hw - 1 || dy == 0 || dy == hh - 1;
            if !is_border {
                continue;
            }

            let wx = hx + dx as i32 + offset_x;
            let wy = hy + dy as i32 + offset_y;
            if let Some((vx, vy)) = viewport.world_to_view(wx, wy) {
                let tx = vx;
                let ty = vy;
                if tx < frame.area().width && ty < frame.area().height {
                    visible_left = Some(visible_left.map_or(tx, |v: u16| v.min(tx)));
                    visible_top = Some(visible_top.map_or(ty, |v: u16| v.min(ty)));
                    visible_right = Some(visible_right.map_or(tx, |v: u16| v.max(tx)));
                    visible_bottom = Some(visible_bottom.map_or(ty, |v: u16| v.max(ty)));
                }
            }
        }
    }

    let Some(left) = visible_left else { return };
    let Some(top) = visible_top else { return };
    let Some(right) = visible_right else { return };
    let Some(bottom) = visible_bottom else { return };

    let rect = Rect::new(
        left,
        top,
        right.saturating_sub(left).saturating_add(1),
        bottom.saturating_sub(top).saturating_add(1),
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(title))
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(block, rect);

    let overlay = Paragraph::new(format!(
        "Frame: {} / {}\nPlaying: {}",
        hero.current_frame,
        hero.frames.len(),
        hero.playing
    ))
    .style(Style::default().fg(Color::Gray).bg(Color::Black));
    frame.render_widget(overlay, Rect::new(0, 0, 28, 2));

    // Keep a subtle marker on the border where the hero center sits.
    let center_x = (hero.x - viewport.x).max(0) as u16;
    let center_y = (hero.y - viewport.y).max(0) as u16;
    if center_x < frame.area().width && center_y < frame.area().height {
        if let Some(cell) = frame.buffer_mut().cell_mut((center_x, center_y)) {
            cell.set_symbol("·").set_fg(Color::Yellow);
        }
    }
}
