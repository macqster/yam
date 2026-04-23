use figlet_rs::FIGlet;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ClockFont {
    Small,
    Standard,
    Fender,
    Gothic,
}

pub struct FontRegistry {
    fonts: HashMap<ClockFont, FIGlet>,
    fallback: FIGlet,
}

impl FontRegistry {
    pub fn new() -> Self {
        let fallback = FIGlet::standard().unwrap_or_else(|_| FIGlet::small().unwrap());
        let assets = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts");
        let mut fonts = HashMap::new();

        fonts.insert(
            ClockFont::Small,
            load_font(&format!("{assets}/Small.flf")).unwrap_or_else(|| fallback.clone()),
        );
        fonts.insert(
            ClockFont::Standard,
            load_font(&format!("{assets}/Standard.flf")).unwrap_or_else(|| fallback.clone()),
        );
        fonts.insert(
            ClockFont::Fender,
            load_font(&format!("{assets}/Fender.flf")).unwrap_or_else(|| fallback.clone()),
        );
        fonts.insert(
            ClockFont::Gothic,
            load_font(&format!("{assets}/Gothic.flf")).unwrap_or_else(|| fallback.clone()),
        );

        Self { fonts, fallback }
    }

    pub fn get(&self, font: ClockFont) -> &FIGlet {
        self.fonts.get(&font).unwrap_or(&self.fallback)
    }

    pub fn display_name(font: ClockFont) -> &'static str {
        match font {
            ClockFont::Small => "small",
            ClockFont::Standard => "standard",
            ClockFont::Fender => "fender",
            ClockFont::Gothic => "gothic",
        }
    }
}

fn load_font(path: &str) -> Option<FIGlet> {
    FIGlet::from_file(path).ok()
}
