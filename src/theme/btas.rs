use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct BtasTheme {
    pub white: Color,
    pub black: Color,
    pub pms_113: Color,
    pub pms_142: Color,
    pub pms_131: Color,
    pub pms_179: Color,
    pub pms_485: Color,
    pub pms_200: Color,
    pub pms_197: Color,
    pub pms_228: Color,
    pub pms_272: Color,
    pub pms_293: Color,
    pub pms_292: Color,
    pub pms_551: Color,
    pub process_cyan: Color,
    pub pms_319: Color,
    pub pms_317: Color,
    pub pms_345: Color,
    pub pms_340: Color,
    pub pms_156: Color,
    pub pms_480: Color,
    pub pms_465: Color,
    pub pms_160: Color,
    pub pms_161: Color,
    pub pms_432: Color,
    pub pms_430: Color,
    pub pms_427: Color,
    pub pms_441: Color,
}

#[allow(dead_code)]
pub const BTAS: BtasTheme = BtasTheme {
    white: Color::Rgb(248, 248, 248),
    black: Color::Rgb(0, 0, 0),
    pms_113: Color::Rgb(248, 214, 0),
    pms_142: Color::Rgb(246, 175, 35),
    pms_131: Color::Rgb(227, 147, 30),
    pms_179: Color::Rgb(210, 40, 48),
    pms_485: Color::Rgb(216, 0, 54),
    pms_200: Color::Rgb(191, 18, 56),
    pms_197: Color::Rgb(241, 156, 178),
    pms_228: Color::Rgb(146, 46, 82),
    pms_272: Color::Rgb(113, 103, 173),
    pms_293: Color::Rgb(0, 71, 178),
    pms_292: Color::Rgb(113, 165, 220),
    pms_551: Color::Rgb(132, 172, 177),
    process_cyan: Color::Rgb(0, 158, 219),
    pms_319: Color::Rgb(83, 193, 170),
    pms_317: Color::Rgb(183, 218, 206),
    pms_345: Color::Rgb(139, 206, 118),
    pms_340: Color::Rgb(0, 138, 66),
    pms_156: Color::Rgb(243, 184, 92),
    pms_480: Color::Rgb(212, 170, 121),
    pms_465: Color::Rgb(199, 161, 113),
    pms_160: Color::Rgb(168, 101, 47),
    pms_161: Color::Rgb(116, 49, 47),
    pms_432: Color::Rgb(45, 53, 58),
    pms_430: Color::Rgb(130, 140, 147),
    pms_427: Color::Rgb(204, 207, 209),
    pms_441: Color::Rgb(200, 209, 192),
};

impl BtasTheme {
    pub fn panel_text(self) -> Style {
        Style::default().bg(self.pms_432).fg(self.pms_427)
    }

    pub fn modal_panel(self) -> Style {
        Style::default().bg(self.pms_432).fg(self.pms_427)
    }

    pub fn hero_overlay(self) -> Style {
        Style::default().fg(self.pms_427).bg(self.black)
    }

    pub fn camera_indicator_track(self) -> Style {
        Style::default()
            .fg(self.pms_293)
            .add_modifier(Modifier::DIM)
    }

    pub fn camera_indicator_thumb(self) -> Style {
        Style::default()
            .fg(self.pms_292)
            .add_modifier(Modifier::BOLD)
    }

    pub fn pointer_probe(self) -> Style {
        Style::default()
            .fg(self.pms_156)
            .add_modifier(Modifier::BOLD)
    }

    pub fn footer_text(self) -> Style {
        Style::default().fg(self.pms_430)
    }

    pub fn debug_text(self) -> Style {
        Style::default().fg(self.pms_340)
    }

    pub fn guide_trace(self) -> Style {
        Style::default().fg(self.pms_430)
    }

    pub fn vine_stem(self, healthy: bool) -> Style {
        if healthy {
            Style::default().fg(self.pms_345)
        } else {
            Style::default().fg(self.pms_465)
        }
    }
}
