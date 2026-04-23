use std::process::Command;
use std::sync::OnceLock;

use ansi_to_tui::IntoText;
use ratatui::text::Line;

const HERO_GIF_PATH: &str = "/Users/maciejkuster/Desktop/hero_gif_1.gif";

static HERO_FRAME: OnceLock<Vec<Line<'static>>> = OnceLock::new();

pub fn render_frame(path: &str, width: u16, height: u16) -> Vec<Line<'static>> {
    let size_arg = format!("{}x{}", width, height);
    let output = Command::new("chafa")
        .arg(path)
        .arg("--size")
        .arg(size_arg)
        .arg("--format=symbols")
        .arg("--symbols=braille")
        .arg("--animate=off")
        .output()
        .unwrap_or_else(|err| panic!("failed to run chafa: {err}"));

    if !output.status.success() {
        return vec![format!("chafa exited with status {}", output.status).into()];
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .into_owned()
        .into_text()
        .expect("failed to convert ANSI text")
        .lines
}

pub fn hero_frame(width: u16, height: u16) -> Vec<Line<'static>> {
    HERO_FRAME
        .get_or_init(|| render_frame(HERO_GIF_PATH, width, height))
        .clone()
}
