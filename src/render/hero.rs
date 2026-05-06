use crate::render::chafa::{HERO_RENDER_HEIGHT, HERO_RENDER_WIDTH};
use crate::scene::viewport::Viewport;
use crate::theme::{glyphs, style as theme_style};
use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::Paragraph,
};

pub struct Hero {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
    pub frames: Vec<Vec<Line<'static>>>,
    pub current_frame: usize,
    pub playing: bool,
    step_once: bool,
}

impl Hero {
    #[cfg_attr(test, allow(dead_code))]
    pub fn new(world_width: usize, world_height: usize) -> Self {
        // The source hero GIF is square (820x820); the terminal render target is intentionally
        // downscaled to a fixed 96x48 cell footprint to compensate for terminal cell aspect.
        let frames = crate::render::chafa::hero_frames(HERO_RENDER_WIDTH, HERO_RENDER_HEIGHT);
        let first_frame = frames
            .first()
            .cloned()
            .unwrap_or_else(|| vec![Line::from("chafa unavailable")]);
        let width = first_frame
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.chars().count() as u16)
                    .sum()
            })
            .max()
            .unwrap_or(0);
        let height = first_frame.len() as u16;
        let base_width = width;
        let base_height = height;
        let normalized_frames = if frames.is_empty() {
            vec![normalize_frame(
                first_frame.clone(),
                base_width,
                base_height,
            )]
        } else {
            frames
                .into_iter()
                .map(|frame| normalize_frame(frame, base_width, base_height))
                .collect()
        };

        Self {
            x: (world_width / 2) as i32,
            y: (world_height / 2) as i32,
            width,
            height,
            frames: normalized_frames,
            current_frame: 0,
            playing: true,
            step_once: false,
        }
    }

    #[cfg(test)]
    pub fn test_stub(world_width: usize, world_height: usize) -> Self {
        let frame = vec![Line::from(vec![Span::styled(
            "stub",
            theme_style::hero_overlay(),
        )])];
        Self {
            x: (world_width / 2) as i32,
            y: (world_height / 2) as i32,
            width: 4,
            height: 1,
            frames: vec![frame],
            current_frame: 0,
            playing: true,
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

    #[allow(dead_code)]
    pub fn debug_rect(&self) -> (i32, i32, u16, u16) {
        let x = self.x - (self.width as i32 / 2);
        let y = self.y - (self.height as i32 / 2);
        (x, y, self.width, self.height)
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn draw_hero_at(
    frame: &mut Frame,
    hero: &Hero,
    start_x: i32,
    start_y: i32,
    offset_x: i32,
    offset_y: i32,
) {
    let start_x = start_x + offset_x;
    let start_y = start_y + offset_y;
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
    let max_width = frame.area().right().saturating_sub(start_x);
    for (i, line) in lines.iter().skip(skip_rows).enumerate() {
        let clipped = clip_line(line, skip_cols);
        let text = clipped
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();
        if text.chars().all(|c| c == ' ') {
            continue;
        }
        frame
            .buffer_mut()
            .set_line(start_x, start_y + i as u16, &clipped, max_width);
    }
}

#[cfg_attr(test, allow(dead_code))]
fn normalize_frame(lines: Vec<Line<'static>>, width: u16, height: u16) -> Vec<Line<'static>> {
    hard_lock_frame(lines, width, height)
}

#[cfg_attr(test, allow(dead_code))]
fn normalize_line(line: Line<'static>, width: u16) -> Line<'static> {
    let mut remaining = width as usize;
    let mut spans = Vec::new();
    for span in line.spans {
        if remaining == 0 {
            break;
        }

        let mut chars = String::new();
        for ch in span.content.chars() {
            let ch = match ch {
                '\0' | '\r' | '\t' => ' ',
                other => other,
            };
            chars.push(ch);
            if chars.chars().count() >= remaining {
                break;
            }
        }

        if chars.is_empty() {
            continue;
        }

        let count = chars.chars().count();
        remaining = remaining.saturating_sub(count);
        spans.push(Span::styled(chars, span.style));
    }

    if remaining > 0 {
        spans.push(Span::raw(" ".repeat(remaining)));
    }

    Line::from(spans)
}

#[cfg_attr(test, allow(dead_code))]
fn padded_line(width: u16) -> Line<'static> {
    Line::from(vec![Span::raw(" ".repeat(width as usize))])
}

#[cfg_attr(test, allow(dead_code))]
fn hard_lock_frame(lines: Vec<Line<'static>>, width: u16, height: u16) -> Vec<Line<'static>> {
    let mut normalized = Vec::with_capacity(height as usize);
    for line in lines.into_iter().take(height as usize) {
        normalized.push(normalize_line(line, width));
    }
    while normalized.len() < height as usize {
        normalized.push(padded_line(width));
    }
    if normalized.len() > height as usize {
        normalized.truncate(height as usize);
    }
    debug_assert_eq!(normalized.len(), height as usize);
    normalized
}

fn clip_line(line: &Line<'static>, skip_cols: usize) -> Line<'static> {
    let mut remaining = skip_cols;
    let mut spans = Vec::new();

    for span in &line.spans {
        let content = span.content.as_ref();
        let content_width = content.chars().count();
        if remaining >= content_width {
            remaining -= content_width;
            continue;
        }

        let clipped = content.chars().skip(remaining).collect::<String>();
        remaining = 0;
        if !clipped.is_empty() {
            spans.push(Span::styled(clipped, span.style));
        }
    }

    Line::from(spans)
}

#[cfg(test)]
mod tests {
    use super::{draw_hero_at, Hero};
    use ratatui::backend::TestBackend;
    use ratatui::style::{Color, Style};
    use ratatui::text::{Line, Span};
    use ratatui::Terminal;

    #[test]
    fn hero_rendering_preserves_span_styles_into_the_terminal_buffer() {
        let backend = TestBackend::new(4, 1);
        let mut terminal = Terminal::new(backend).expect("terminal should initialize");
        let mut hero = Hero {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
            frames: vec![vec![Line::from(vec![Span::styled(
                "RGB ",
                Style::default().fg(Color::Rgb(114, 22, 15)),
            )])]],
            current_frame: 0,
            playing: true,
            step_once: false,
        };

        terminal
            .draw(|frame| draw_hero_at(frame, &hero, 0, 0, 0, 0))
            .expect("hero line should render");

        let buffer = terminal.backend().buffer();
        assert_eq!(buffer.content[0].symbol(), "R");
        assert_eq!(buffer.content[1].symbol(), "G");
        assert_eq!(buffer.content[0].style().fg, Some(Color::Rgb(114, 22, 15)));
        assert_eq!(buffer.content[1].style().fg, Some(Color::Rgb(114, 22, 15)));

        hero.step_animation();
        assert!(hero.playing);
    }
}

#[allow(dead_code)]
pub fn draw_hero_debug(
    frame: &mut Frame,
    hero: &Hero,
    viewport: &Viewport,
    _offset_x: i32,
    _offset_y: i32,
) {
    let center_x = (hero.x - viewport.x).max(0) as u16;
    let center_y = (hero.y - viewport.y).max(0) as u16;
    if center_x < frame.area().width && center_y < frame.area().height {
        if let Some(cell) = frame.buffer_mut().cell_mut((center_x, center_y)) {
            cell.set_symbol(glyphs::HERO_CENTER_MARKER)
                .set_fg(crate::theme::palette::MARKER);
        }
    }

    let overlay = Paragraph::new(format!(
        "Frame: {} / {}\nPlaying: {}",
        hero.current_frame,
        hero.frames.len(),
        hero.playing
    ))
    .style(theme_style::hero_overlay());
    frame.render_widget(overlay, Rect::new(0, 0, 28, 2));
}

#[allow(dead_code)]
pub fn draw_hero_debug_at(
    _frame: &mut Frame,
    _hero: &Hero,
    _start_x: i32,
    _start_y: i32,
    _offset_x: i32,
    _offset_y: i32,
) {
}
