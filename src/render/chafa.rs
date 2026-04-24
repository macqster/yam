use std::{fs, process::Command};

use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage};
use ratatui::text::Line;
use tempfile::NamedTempFile;

const HERO_GIF_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif");

pub fn render_frame(path: &str, width: u16, height: u16) -> Vec<Line<'static>> {
    let size_arg = format!("{}x{}", width, height);
    let output = Command::new("chafa")
        .arg(path)
        .arg("--size")
        .arg(size_arg)
        .arg("--format=symbols")
        .arg("--symbols=braille")
        .arg("--colors=full")
        .arg("--color-space=din99d")
        .arg("--color-extractor=median")
        .arg("--dither=diffusion")
        .arg("--fg-only")
        .arg("--bg=#100100")
        .arg("--animate=off")
        .output()
        .unwrap_or_else(|err| panic!("failed to run chafa: {err}"));

    if !output.status.success() {
        return vec![format!("chafa exited with status {}", output.status).into()];
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .map(|line| Line::from(line.to_string()))
        .collect()
}

pub fn hero_frames(width: u16, height: u16) -> Vec<Vec<Line<'static>>> {
    let frames = decode_gif_frames(HERO_GIF_PATH);
    frames
        .into_iter()
        .map(|frame| render_image_frame(&frame, width, height))
        .collect()
}

fn decode_gif_frames(path: &str) -> Vec<DynamicImage> {
    let file = fs::File::open(path).unwrap_or_else(|err| panic!("failed to open gif {path}: {err}"));
    let reader = std::io::BufReader::new(file);
    let decoder = GifDecoder::new(reader)
        .unwrap_or_else(|err| panic!("failed to decode gif {path}: {err}"));
    let frames = decoder
        .into_frames()
        .collect_frames()
        .unwrap_or_else(|err| panic!("failed to collect gif frames from {path}: {err}"));
    frames
        .into_iter()
        .map(|frame| DynamicImage::ImageRgba8(frame.into_buffer()))
        .collect()
}

fn render_image_frame(image: &DynamicImage, width: u16, height: u16) -> Vec<Line<'static>> {
    let temp = NamedTempFile::new().unwrap_or_else(|err| panic!("failed to create temp file: {err}"));
    let temp_path = temp.path().to_path_buf();
    image
        .save(&temp_path)
        .unwrap_or_else(|err| panic!("failed to write temp image {temp_path:?}: {err}"));
    let rendered = render_frame(
        temp_path
            .to_str()
            .unwrap_or_else(|| panic!("temp path not utf-8: {temp_path:?}")),
        width,
        height,
    );
    rendered
}
