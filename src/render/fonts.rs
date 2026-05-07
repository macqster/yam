use figlet_rs::FIGlet;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FigletFontId {
    Small,
    Standard,
    Fender,
    Gothic,
}

pub type ClockFont = FigletFontId;

impl FigletFontId {
    #[cfg(test)]
    pub fn all() -> &'static [Self] {
        &[Self::Small, Self::Standard, Self::Fender, Self::Gothic]
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Standard => "standard",
            Self::Fender => "fender",
            Self::Gothic => "gothic",
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "small" => Self::Small,
            "standard" => Self::Standard,
            "fender" => Self::Fender,
            "gothic" => Self::Gothic,
            _ => Self::Gothic,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Small => Self::Standard,
            Self::Standard => Self::Fender,
            Self::Fender => Self::Gothic,
            Self::Gothic => Self::Small,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Small => Self::Gothic,
            Self::Standard => Self::Small,
            Self::Fender => Self::Standard,
            Self::Gothic => Self::Fender,
        }
    }
}

const SMALL_FLF: &str = include_str!("../../assets/fonts/Small.flf");
const STANDARD_FLF: &str = include_str!("../../assets/fonts/Standard.flf");
const FENDER_FLF: &str = include_str!("../../assets/fonts/Fender.flf");
const GOTHIC_FLF: &str = include_str!("../../assets/fonts/Gothic.flf");

const BUNDLED_FONTS: &[(FigletFontId, &str)] = &[
    (FigletFontId::Small, SMALL_FLF),
    (FigletFontId::Standard, STANDARD_FLF),
    (FigletFontId::Fender, FENDER_FLF),
    (FigletFontId::Gothic, GOTHIC_FLF),
];

pub struct FontRegistry {
    fonts: HashMap<FigletFontId, FIGlet>,
    fallback: FIGlet,
}

impl FontRegistry {
    pub fn new() -> Self {
        let fallback = FIGlet::from_content(STANDARD_FLF)
            .or_else(|_| FIGlet::standard())
            .or_else(|_| FIGlet::small())
            .expect("a fallback FIGlet font should be available");

        let mut fonts = HashMap::new();
        for (id, content) in BUNDLED_FONTS {
            let font = FIGlet::from_content(content).unwrap_or_else(|_| fallback.clone());
            fonts.insert(*id, font);
        }

        Self { fonts, fallback }
    }

    pub fn get(&self, font: FigletFontId) -> &FIGlet {
        self.fonts.get(&font).unwrap_or(&self.fallback)
    }

    pub fn render(&self, font: FigletFontId, text: &str) -> Vec<String> {
        crate::render::figlet::render_figlet(self.get(font), text)
    }

    #[cfg(test)]
    pub fn list(&self) -> Vec<FigletFontId> {
        FigletFontId::all().to_vec()
    }
}

impl Default for FontRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{FigletFontId, FontRegistry};

    #[test]
    fn bundled_fonts_load_and_render() {
        let fonts = FontRegistry::new();
        for id in fonts.list() {
            let lines = fonts.render(id, "yam");
            assert!(!lines.is_empty());
            assert!(lines.iter().any(|line| !line.trim().is_empty()));
        }
    }

    #[test]
    fn figlet_font_names_round_trip() {
        for id in FigletFontId::all() {
            assert_eq!(FigletFontId::from_name(id.display_name()), *id);
        }
    }
}
