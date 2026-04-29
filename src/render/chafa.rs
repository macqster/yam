use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use ansi_to_tui::IntoText;
use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage, ImageDecoder, ImageFormat};
use image::{Rgba, RgbaImage};
use ratatui::text::{Line, Text};

const HERO_GIF_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif");
const HERO_FRAME_BG: Rgba<u8> = Rgba([16, 1, 0, 255]);
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
    use super::{decode_gif_frames, hero_frames, HERO_RENDER_HEIGHT, HERO_RENDER_WIDTH};

    #[test]
    fn hero_frame_buffer_has_multiple_frames() {
        let frames = hero_frames(HERO_RENDER_WIDTH, HERO_RENDER_HEIGHT);
        assert!(frames.len() > 1, "expected multiple hero frames");
    }

    #[test]
    fn decoded_hero_frames_keep_full_canvas_geometry() {
        let frames = decode_gif_frames(super::HERO_GIF_PATH);
        assert_eq!(frames.len(), 64);
        for frame_index in [0, 1, 15, 19, 30, 63] {
            assert_eq!(
                frames[frame_index].width(),
                820,
                "frame {frame_index} width"
            );
            assert_eq!(
                frames[frame_index].height(),
                820,
                "frame {frame_index} height"
            );
            assert_eq!(
                frames[frame_index].to_rgba8().get_pixel(0, 0)[3],
                255,
                "frame {frame_index} must be flattened to an opaque canvas"
            );
        }
    }
}

fn decode_gif_frames(path: &str) -> Vec<DynamicImage> {
    let file =
        fs::File::open(path).unwrap_or_else(|err| panic!("failed to open gif {path}: {err}"));
    let reader = std::io::BufReader::new(file);
    let decoder =
        GifDecoder::new(reader).unwrap_or_else(|err| panic!("failed to decode gif {path}: {err}"));
    let canvas = decoder.dimensions();
    let frames = decoder
        .into_frames()
        .collect_frames()
        .unwrap_or_else(|err| panic!("failed to collect gif frames from {path}: {err}"));
    frames
        .into_iter()
        .map(|frame| DynamicImage::ImageRgba8(frame_to_canvas(frame, canvas)))
        .collect()
}

fn frame_to_canvas(frame: image::Frame, canvas: (u32, u32)) -> RgbaImage {
    let mut image = RgbaImage::from_pixel(canvas.0, canvas.1, HERO_FRAME_BG);
    let left = frame.left();
    let top = frame.top();
    for (x, y, pixel) in frame.into_buffer().enumerate_pixels() {
        let target_x = left + x;
        let target_y = top + y;
        if target_x < canvas.0 && target_y < canvas.1 {
            image.put_pixel(target_x, target_y, flatten_pixel(*pixel));
        }
    }
    image
}

fn flatten_pixel(pixel: Rgba<u8>) -> Rgba<u8> {
    let alpha = pixel[3] as u16;
    if alpha == 255 {
        return pixel;
    }
    if alpha == 0 {
        return HERO_FRAME_BG;
    }

    let inv_alpha = 255 - alpha;
    let blend =
        |fg: u8, bg: u8| -> u8 { (((fg as u16 * alpha) + (bg as u16 * inv_alpha)) / 255) as u8 };

    Rgba([
        blend(pixel[0], HERO_FRAME_BG[0]),
        blend(pixel[1], HERO_FRAME_BG[1]),
        blend(pixel[2], HERO_FRAME_BG[2]),
        255,
    ])
}

fn prepare_temp_frame_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temp_dir =
        std::env::temp_dir().join(format!("yam_rust_frames_{}_{}", std::process::id(), unique));
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
