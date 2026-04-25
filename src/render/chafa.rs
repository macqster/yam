use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use ansi_to_tui::IntoText;
use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage, ImageFormat};
use ratatui::text::{Line, Text};

const HERO_GIF_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif");
pub const HERO_RENDER_WIDTH: u16 = 96;
pub const HERO_RENDER_HEIGHT: u16 = 48;

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

    let text: Text<'static> = output
        .stdout
        .as_slice()
        .into_text()
        .unwrap_or_else(|_| Text::raw("ANSI_PARSE_ERROR"));
    text.lines
}

pub fn hero_frames(width: u16, height: u16) -> Vec<Vec<Line<'static>>> {
    let frames = decode_gif_frames(HERO_GIF_PATH);
    let temp_dir = prepare_temp_frame_dir();
    frames
        .into_iter()
        .enumerate()
        .map(|(frame_index, frame)| {
            render_image_frame(&temp_dir, frame_index, &frame, width, height)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{hero_frames, HERO_RENDER_HEIGHT, HERO_RENDER_WIDTH};

    #[test]
    fn hero_frame_buffer_has_multiple_frames() {
        let frames = hero_frames(HERO_RENDER_WIDTH, HERO_RENDER_HEIGHT);
        println!("LOADED FRAME COUNT: {}", frames.len());
        assert!(frames.len() > 1, "expected multiple hero frames");
    }
}

fn decode_gif_frames(path: &str) -> Vec<DynamicImage> {
    let file =
        fs::File::open(path).unwrap_or_else(|err| panic!("failed to open gif {path}: {err}"));
    let reader = std::io::BufReader::new(file);
    let decoder =
        GifDecoder::new(reader).unwrap_or_else(|err| panic!("failed to decode gif {path}: {err}"));
    let frames = decoder
        .into_frames()
        .collect_frames()
        .unwrap_or_else(|err| panic!("failed to collect gif frames from {path}: {err}"));
    frames
        .into_iter()
        .map(|frame| DynamicImage::ImageRgba8(frame.into_buffer()))
        .collect()
}

fn prepare_temp_frame_dir() -> PathBuf {
    let temp_dir = std::env::temp_dir().join("yam_rust_frames");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .unwrap_or_else(|err| panic!("failed to create temp frame dir {temp_dir:?}: {err}"));
    temp_dir
}

fn render_image_frame(
    temp_dir: &Path,
    frame_index: usize,
    image: &DynamicImage,
    width: u16,
    height: u16,
) -> Vec<Line<'static>> {
    let temp_path = temp_dir.join(format!("yam_frame_{frame_index:04}.png"));
    image
        .save_with_format(&temp_path, ImageFormat::Png)
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
