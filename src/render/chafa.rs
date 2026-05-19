use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use ansi_to_tui::IntoText;
use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage, ImageDecoder, ImageFormat};
use image::{Rgba, RgbaImage};
use ratatui::text::{Line, Text};

use crate::render::hero_cache::{load_hero_frame_set, save_hero_frame_set, HeroFrameSet};

const HERO_GIF_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif");
const HERO_FRAME_BG: Rgba<u8> = Rgba([16, 1, 0, 255]);
pub const HERO_RENDER_WIDTH: u16 = 96;
pub const HERO_RENDER_HEIGHT: u16 = 48;

pub fn render_frame(path: &str, width: u16, height: u16) -> Vec<Line<'static>> {
    render_frame_with_command("chafa", path, width, height)
}

fn render_frame_with_command(
    command: &str,
    path: &str,
    width: u16,
    height: u16,
) -> Vec<Line<'static>> {
    let size_arg = format!("{}x{}", width, height);
    let output = match chafa_output(command, path, &size_arg) {
        Ok(output) => output,
        Err(err) => return vec![format!("chafa unavailable: {err}").into()],
    };

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

fn chafa_output(command: &str, path: &str, size_arg: &str) -> std::io::Result<Output> {
    Command::new(command)
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
}

pub fn hero_frames(width: u16, height: u16) -> Vec<Vec<Line<'static>>> {
    let frames = match decode_gif_frames(HERO_GIF_PATH) {
        Ok(frames) => frames,
        Err(err) => return vec![vec![format!("hero gif unavailable: {err}").into()]],
    };
    let temp_dir = match TempFrameDir::new() {
        Ok(temp_dir) => temp_dir,
        Err(err) => return vec![vec![format!("hero temp dir unavailable: {err}").into()]],
    };
    frames
        .into_iter()
        .enumerate()
        .map(|(frame_index, frame)| {
            render_image_frame(temp_dir.path(), frame_index, &frame, width, height)
                .unwrap_or_else(|err| vec![format!("hero frame render failed: {err}").into()])
        })
        .collect()
}

pub fn hero_frames_cached(width: u16, height: u16) -> Vec<Vec<Line<'static>>> {
    let cache_path = hero_frame_cache_path(width, height);
    if let Some(frame_set) = load_cached_hero_frames(&cache_path, width, height) {
        return frame_set.to_lines();
    }

    let frames = hero_frames(width, height);
    if hero_frames_are_cacheable(&frames) {
        let frame_set = HeroFrameSet::from_lines(width, height, &frames);
        let _ = save_hero_frame_set(&cache_path, &frame_set);
    }
    frames
}

#[cfg(test)]
mod tests {
    use super::{
        cache_is_fresh_against, decode_gif_frames, hero_frames, render_frame_with_command,
        tone_lift_dark_reds, HERO_RENDER_HEIGHT, HERO_RENDER_WIDTH,
    };
    use image::Rgba;
    use ratatui::text::Line;
    use std::{fs, thread, time::Duration};

    #[test]
    fn hero_frame_buffer_has_multiple_frames() {
        let frames = hero_frames(HERO_RENDER_WIDTH, HERO_RENDER_HEIGHT);
        assert!(frames.len() > 1, "expected multiple hero frames");
    }

    #[test]
    fn decoded_hero_frames_keep_full_canvas_geometry() {
        let frames = decode_gif_frames(super::HERO_GIF_PATH).expect("decode hero gif");
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

    #[test]
    fn temp_frame_dir_is_removed_on_drop() {
        let path = {
            let temp_dir = super::TempFrameDir::new().expect("temp frame dir");
            let path = temp_dir.path().to_path_buf();
            assert!(
                path.exists(),
                "temp frame dir should exist during render batch"
            );
            path
        };

        assert!(
            !path.exists(),
            "temp frame dir should be removed when the render batch ends"
        );
    }

    #[test]
    fn missing_hero_gif_returns_decode_error_instead_of_panicking() {
        let err = decode_gif_frames("__yam_missing_hero.gif").expect_err("missing gif should fail");
        assert!(err.contains("failed to open gif"));
    }

    #[test]
    fn placeholder_hero_frames_are_not_cacheable() {
        let frames = vec![vec![Line::from("chafa unavailable: missing")]];
        assert!(!super::hero_frames_are_cacheable(&frames));

        let frames = vec![vec![Line::from("hero gif unavailable: missing")]];
        assert!(!super::hero_frames_are_cacheable(&frames));
    }

    #[test]
    fn render_frame_returns_placeholder_when_chafa_is_unavailable() {
        let lines =
            render_frame_with_command("__yam_missing_chafa_binary__", super::HERO_GIF_PATH, 4, 2);
        assert_eq!(lines.len(), 1);
        let text = lines[0]
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();
        assert!(text.starts_with("chafa unavailable:"));
    }

    #[test]
    fn cache_freshness_accepts_newer_cache_file() {
        let temp_dir = tempfile::tempdir().expect("tempdir");
        let source = temp_dir.path().join("hero.gif");
        let cache = temp_dir.path().join("hero.frame_cache.json");

        fs::write(&source, b"source").expect("write source");
        thread::sleep(Duration::from_millis(5));
        fs::write(&cache, b"cache").expect("write cache");

        assert!(cache_is_fresh_against(&cache, &source));
    }

    #[test]
    fn cache_freshness_rejects_stale_cache_file() {
        let temp_dir = tempfile::tempdir().expect("tempdir");
        let source = temp_dir.path().join("hero.gif");
        let cache = temp_dir.path().join("hero.frame_cache.json");

        fs::write(&cache, b"cache").expect("write cache");
        thread::sleep(Duration::from_millis(5));
        fs::write(&source, b"source").expect("write source");

        assert!(!cache_is_fresh_against(&cache, &source));
    }
}

fn decode_gif_frames(path: &str) -> Result<Vec<DynamicImage>, String> {
    let file = fs::File::open(path).map_err(|err| format!("failed to open gif {path}: {err}"))?;
    let reader = std::io::BufReader::new(file);
    let decoder =
        GifDecoder::new(reader).map_err(|err| format!("failed to decode gif {path}: {err}"))?;
    let canvas = decoder.dimensions();
    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(|err| format!("failed to collect gif frames from {path}: {err}"))?;
    Ok(frames
        .into_iter()
        .map(|frame| DynamicImage::ImageRgba8(frame_to_canvas(frame, canvas)))
        .collect())
}

fn load_cached_hero_frames(path: &Path, width: u16, height: u16) -> Option<HeroFrameSet> {
    if !hero_cache_is_fresh(path) {
        return None;
    }

    let frame_set = load_hero_frame_set(path).ok()?;
    if frame_set.render_width != width || frame_set.render_height != height {
        return None;
    }
    if frame_set.frames.is_empty() {
        return None;
    }
    Some(frame_set)
}

fn hero_frames_are_cacheable(frames: &[Vec<Line<'static>>]) -> bool {
    !frames.is_empty() && !frames.iter().any(|frame| is_placeholder_frame(frame))
}

fn is_placeholder_frame(frame: &[Line<'static>]) -> bool {
    if frame.len() != 1 {
        return false;
    }

    let text = frame[0]
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect::<String>();
    text.starts_with("chafa unavailable:")
        || text.starts_with("chafa exited with status")
        || text.starts_with("hero gif unavailable:")
        || text.starts_with("hero temp dir unavailable:")
        || text.starts_with("hero frame render failed:")
}

fn hero_cache_is_fresh(path: &Path) -> bool {
    cache_is_fresh_against(path, Path::new(HERO_GIF_PATH))
}

fn cache_is_fresh_against(cache_path: &Path, source_path: &Path) -> bool {
    let cache_meta = match fs::metadata(cache_path) {
        Ok(meta) => meta,
        Err(_) => return false,
    };
    let gif_meta = match fs::metadata(source_path) {
        Ok(meta) => meta,
        Err(_) => return false,
    };

    match (cache_meta.modified(), gif_meta.modified()) {
        (Ok(cache_modified), Ok(gif_modified)) => cache_modified >= gif_modified,
        _ => false,
    }
}

fn hero_frame_cache_path(width: u16, height: u16) -> PathBuf {
    hero_cache_dir().join(format!("hero_gif_1.{width}x{height}.frame_cache.json"))
}

fn hero_cache_dir() -> PathBuf {
    if let Some(path) = env::var_os("XDG_CACHE_HOME") {
        return PathBuf::from(path).join("yam");
    }

    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home).join(".cache").join("yam");
    }

    env::temp_dir().join("yam")
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

fn prepare_temp_frame_dir() -> std::io::Result<PathBuf> {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temp_dir =
        std::env::temp_dir().join(format!("yam_rust_frames_{}_{}", std::process::id(), unique));
    fs::create_dir_all(&temp_dir)?;
    Ok(temp_dir)
}

struct TempFrameDir {
    path: PathBuf,
}

impl TempFrameDir {
    fn new() -> std::io::Result<Self> {
        prepare_temp_frame_dir().map(|path| Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempFrameDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn render_image_frame(
    temp_dir: &Path,
    frame_index: usize,
    image: &DynamicImage,
    width: u16,
    height: u16,
) -> Result<Vec<Line<'static>>, String> {
    let temp_path = temp_dir.join(format!("yam_frame_{frame_index:04}.png"));
    image
        .save_with_format(&temp_path, ImageFormat::Png)
        .map_err(|err| format!("failed to write temp image {temp_path:?}: {err}"))?;
    let temp_path = temp_path
        .to_str()
        .ok_or_else(|| format!("temp path not utf-8: {temp_path:?}"))?;
    let rendered = render_frame(temp_path, width, height);
    Ok(rendered)
}
