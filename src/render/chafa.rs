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
        .arg("--color-space=rgb")
        .arg("--color-extractor=average")
        .arg("--dither=none")
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
    use super::{
        decode_gif_frames, hero_frames, tone_lift_dark_reds, HERO_RENDER_HEIGHT, HERO_RENDER_WIDTH,
    };
    use image::Rgba;

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

    #[test]
    fn dark_reds_get_lifted_before_chafa_conversion() {
        let lifted = tone_lift_dark_reds(Rgba([114, 22, 15, 255]));
        assert!(lifted[0] >= 114);
        assert!(lifted[1] >= 22);
        assert!(lifted[2] >= 15);
        assert!(lifted[0] <= 132);
        assert!(lifted[1] <= 34);
        assert!(lifted[2] <= 26);
        assert_eq!(lifted[3], 255);
    }

    #[test]
    fn neutral_dark_pixels_stay_neutral() {
        let pixel = Rgba([18, 18, 18, 255]);
        assert_eq!(tone_lift_dark_reds(pixel), pixel);
    }

    #[test]
    fn warm_skin_tones_stay_neutral() {
        let pixel = Rgba([180, 120, 90, 255]);
        assert_eq!(tone_lift_dark_reds(pixel), pixel);
    }

    #[test]
    fn bright_orange_tones_stay_neutral() {
        let pixel = Rgba([200, 40, 20, 255]);
        assert_eq!(tone_lift_dark_reds(pixel), pixel);
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
            image.put_pixel(
                target_x,
                target_y,
                tone_lift_dark_reds(flatten_pixel(*pixel)),
            );
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
fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let saturation = if max == 0.0 { 0.0 } else { delta / max };
    (hue.rem_euclid(360.0), saturation, max)
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (u8, u8, u8) {
    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = value - c;

    let (r1, g1, b1) = match hue {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let to_u8 = |channel: f32| ((channel + m).clamp(0.0, 1.0) * 255.0).round() as u8;
    (to_u8(r1), to_u8(g1), to_u8(b1))
}

fn tone_lift_dark_reds(pixel: Rgba<u8>) -> Rgba<u8> {
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];
    let (hue, saturation, value) = rgb_to_hsv(r, g, b);

    if !is_dark_red(hue, saturation, value) {
        return pixel;
    }

    let value = (value + 0.08).min(0.45);
    let saturation = (saturation * 1.02).min(1.0);
    let (r, g, b) = hsv_to_rgb(hue, saturation, value);
    Rgba([r, g, b, 255])
}

fn is_dark_red(hue: f32, saturation: f32, value: f32) -> bool {
    let red_hue = hue <= 20.0 || hue >= 340.0;
    red_hue && saturation >= 0.45 && value <= 0.42
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
