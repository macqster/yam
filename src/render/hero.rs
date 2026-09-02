use crate::render::hero_source::{self, HeroSource};
use ratatui::text::{Line, Span};

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
    /// Build the hero the runtime should show.
    ///
    /// The source is resolved rather than hard-wired to `DEFAULT`, so
    /// `YAM_HERO_SOURCE=<stem>` can point a normal launch at any registered
    /// asset. Unset or unknown resolves back to `DEFAULT`.
    #[cfg_attr(test, allow(dead_code))]
    pub fn new(world_width: usize, world_height: usize) -> Self {
        Self::from_source(&hero_source::resolve_from_env(), world_width, world_height)
    }

    /// Build a hero from an explicit source asset.
    ///
    /// The emitted cell footprint is measured from the rendered frames rather
    /// than assumed from `source.render_width`/`render_height`: chafa preserves
    /// the source's aspect ratio, so the requested size is an upper bound, not
    /// a guarantee. Every frame is then hard-locked to frame 0's measured
    /// footprint so animation cannot shift the hero's geometry mid-loop.
    #[cfg_attr(test, allow(dead_code))]
    pub fn from_source(source: &HeroSource, world_width: usize, world_height: usize) -> Self {
        let frames = crate::render::chafa::hero_frames_cached_from(
            source,
            source.render_width,
            source.render_height,
        );
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
        let frame = vec![Line::from(vec![Span::raw("stub")])];
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

#[cfg(test)]
mod tests {
    use super::Hero;
    use ratatui::text::Line;

    /// The previous test here drove `draw_hero_at`, part of a direct-to-`Frame`
    /// drawing API that the grid/layer pipeline replaced; it was removed with
    /// that API. Its style-preservation claim is covered on the live path by
    /// `render::cell_grid`'s round-trip test. Its one assertion about live
    /// behavior was `step_animation`, so that is what this module now covers
    /// properly - `tick`, `toggle_animation`, and `step_animation` previously
    /// had no direct tests at all.
    fn hero_with_frames(count: usize) -> Hero {
        Hero {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            frames: (0..count)
                .map(|i| vec![Line::from(i.to_string())])
                .collect(),
            current_frame: 0,
            playing: true,
            step_once: false,
        }
    }

    #[test]
    fn playing_hero_advances_and_wraps_on_tick() {
        let mut hero = hero_with_frames(3);
        hero.tick();
        assert_eq!(hero.current_frame, 1);
        hero.tick();
        assert_eq!(hero.current_frame, 2);
        hero.tick();
        assert_eq!(hero.current_frame, 0, "playing playback wraps");
    }

    #[test]
    fn paused_hero_holds_its_frame() {
        let mut hero = hero_with_frames(3);
        hero.toggle_animation();
        assert!(!hero.playing);
        hero.tick();
        hero.tick();
        assert_eq!(hero.current_frame, 0);
    }

    #[test]
    fn step_advances_exactly_one_frame_while_paused() {
        let mut hero = hero_with_frames(3);
        hero.toggle_animation();
        hero.step_animation();
        hero.tick();
        assert_eq!(hero.current_frame, 1);
        // The step is consumed, so a further tick must not advance again.
        hero.tick();
        assert_eq!(hero.current_frame, 1);
    }

    #[test]
    fn stepping_does_not_wrap_at_the_last_frame() {
        // Deliberate asymmetry with playing mode, which does wrap: stepping is
        // for inspecting frames, so it stops at the end rather than looping.
        let mut hero = hero_with_frames(2);
        hero.toggle_animation();
        for _ in 0..4 {
            hero.step_animation();
            hero.tick();
        }
        assert_eq!(hero.current_frame, 1);
    }

    #[test]
    fn step_is_ignored_while_playing() {
        let mut hero = hero_with_frames(3);
        hero.step_animation();
        assert!(!hero.step_once, "a playing hero has nothing to step");
    }

    #[test]
    fn resuming_playback_clears_a_pending_step() {
        let mut hero = hero_with_frames(3);
        hero.toggle_animation();
        hero.step_animation();
        assert!(hero.step_once);
        hero.toggle_animation();
        assert!(!hero.step_once);
    }

    #[test]
    fn frame_falls_back_to_the_first_frame_when_the_index_is_stale() {
        let mut hero = hero_with_frames(2);
        hero.current_frame = 99;
        assert_eq!(hero.frame(), &vec![Line::from("0".to_string())]);
    }
}
