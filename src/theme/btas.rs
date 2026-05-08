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
    white: Color::Rgb(230, 232, 235),
    black: Color::Rgb(0, 0, 0),
    pms_113: Color::Rgb(248, 214, 0),
    pms_142: Color::Rgb(196, 138, 44),
    pms_131: Color::Rgb(166, 124, 52),
    pms_179: Color::Rgb(178, 78, 46),
    pms_485: Color::Rgb(163, 58, 50),
    pms_200: Color::Rgb(191, 18, 56),
    pms_197: Color::Rgb(241, 156, 178),
    pms_228: Color::Rgb(146, 46, 82),
    pms_272: Color::Rgb(113, 103, 173),
    pms_293: Color::Rgb(44, 76, 138),
    pms_292: Color::Rgb(53, 95, 168),
    pms_551: Color::Rgb(132, 172, 177),
    process_cyan: Color::Rgb(0, 158, 219),
    pms_319: Color::Rgb(83, 193, 170),
    pms_317: Color::Rgb(105, 192, 144),
    pms_345: Color::Rgb(79, 142, 108),
    pms_340: Color::Rgb(62, 115, 87),
    pms_156: Color::Rgb(196, 138, 44),
    pms_480: Color::Rgb(178, 78, 46),
    pms_465: Color::Rgb(166, 124, 52),
    pms_160: Color::Rgb(168, 101, 47),
    pms_161: Color::Rgb(116, 49, 47),
    pms_432: Color::Rgb(30, 33, 36),
    pms_430: Color::Rgb(160, 166, 173),
    pms_427: Color::Rgb(230, 232, 235),
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
}
