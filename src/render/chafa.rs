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

pub(crate) const HERO_GIF_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif");
const HERO_DISPLAY_BG: Rgba<u8> = Rgba([16, 1, 0, 255]);
// Bump whenever the renderer, ANSI conversion, or serialized frame contract
// changes; GIF mtimes alone cannot invalidate those cached frames.
// r3: `tone_lift_dark_reds` was removed from `frame_to_canvas`, which changes
// every rendered pixel. Without this bump an `r2` cache written while the tone
// lift was still live would keep being served as trusted art and the removal
// would be invisible.
const HERO_CACHE_REVISION: u8 = 3;
pub const HERO_RENDER_WIDTH: u16 = 96;
pub const HERO_RENDER_HEIGHT: u16 = 48;

/// Human-readable name for the exact chafa preset below. Bump this whenever
/// `chafa_preset_args()` changes in a way that affects visible output --
/// it is recorded verbatim in every compiled `HeroManifest`
/// (`render::hero_manifest`) so a package's rendering intent is legible
/// without cross-referencing this file.
pub(crate) const HERO_PRESET_ID: &str = "rgb-median-fgonly-braille-v1";

/// The single authoritative chafa preset, shared by ordinary runtime
/// rendering (`chafa_output`) and the offline compiler
/// (`render::hero_compiler`), so the two paths cannot silently drift apart.
/// Excludes the input path and `--size`, which callers supply per frame.
pub(crate) fn chafa_preset_args() -> Vec<String> {
    vec![
        "--format=symbols".to_string(),
        "--symbols=braille".to_string(),
        "--colors=full".to_string(),
        "--color-space=rgb".to_string(),
        "--color-extractor=median".to_string(),
        "--dither=none".to_string(),
        "--fg-only".to_string(),
        format!(
            "--bg=#{:02x}{:02x}{:02x}",
            HERO_DISPLAY_BG[0], HERO_DISPLAY_BG[1], HERO_DISPLAY_BG[2]
        ),
        "--animate=off".to_string(),
    ]
}

/// Best-effort captured compiler version string for manifest provenance.
/// Returns `"unknown"` rather than failing when the binary is missing or
/// `--version` cannot be parsed; version capture should never be the reason
/// an offline compile run fails.
pub(crate) fn chafa_version(command: &str) -> String {
    match Command::new(command).arg("--version").output() {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("unknown")
            .trim()
            .to_string(),
        _ => "unknown".to_string(),
    }
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
    let mut cmd = Command::new(command);
    cmd.arg(path).arg("--size").arg(size_arg);
    for arg in chafa_preset_args() {
        cmd.arg(arg);
    }
    cmd.output()
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

fn decode_gif_raw(path: &str) -> Result<(Vec<image::Frame>, (u32, u32)), String> {
    let file = fs::File::open(path).map_err(|err| format!("failed to open gif {path}: {err}"))?;
    let reader = std::io::BufReader::new(file);
    let decoder =
        GifDecoder::new(reader).map_err(|err| format!("failed to decode gif {path}: {err}"))?;
    let canvas = decoder.dimensions();
    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(|err| format!("failed to collect gif frames from {path}: {err}"))?;
    Ok((frames, canvas))
}

fn decode_gif_frames(path: &str) -> Result<Vec<DynamicImage>, String> {
    let (frames, canvas) = decode_gif_raw(path)?;
    Ok(frames
        .into_iter()
        .map(|frame| DynamicImage::ImageRgba8(frame_to_canvas(frame, canvas)))
        .collect())
}

/// Like `decode_gif_frames`, but also returns each frame's authored delay
/// in milliseconds, for `render::hero_compiler` to record in a
/// `HeroManifest`. The ordinary runtime hero path does not need per-frame
/// timing today (`Hero::tick()` uses a fixed FPS), so this stays a separate
/// entry point rather than changing `decode_gif_frames`'s signature.
pub(crate) fn decode_gif_frames_with_delays(
    path: &str,
) -> Result<Vec<(DynamicImage, u32)>, String> {
    let (frames, canvas) = decode_gif_raw(path)?;
    Ok(frames
        .into_iter()
        .map(|frame| {
            let delay_ms = std::time::Duration::from(frame.delay()).as_millis() as u32;
            let image = DynamicImage::ImageRgba8(frame_to_canvas(frame, canvas));
            (image, delay_ms)
        })
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
    hero_cache_dir().join(format!(
        "hero_gif_1.r{HERO_CACHE_REVISION}.{width}x{height}.frame_cache.json"
    ))
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
    // No pixel-side color correction happens here by design: dark-color
    // fidelity is owned by the source art and the chafa preset (see
    // docs/hero-revision.md), not by a code-side tone lift. A prior
    // hue/saturation/value dark-red lift lived here and was removed 2026-07-27
    // ahead of the vector-redrawn source it anticipates, on the bet that a
    // clean source needs no per-pixel compensation. That source has NOT landed
    // yet -- `assets/hero_gif_1.gif` is still the 2026-07-22 raster original --
    // so the removal is currently unverified against rendered output. Confirm
    // with a live `scripts/tmux-smoke.sh` A/B before treating it as settled.
    let mut image = RgbaImage::from_pixel(canvas.0, canvas.1, Rgba([0, 0, 0, 0]));
    let left = frame.left();
    let top = frame.top();
    for (x, y, pixel) in frame.into_buffer().enumerate_pixels() {
        let target_x = left + x;
        let target_y = top + y;
        if target_x < canvas.0 && target_y < canvas.1 {
            image.put_pixel(target_x, target_y, *pixel);
        }
    }
    image
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

pub(crate) struct TempFrameDir {
    path: PathBuf,
}

impl TempFrameDir {
    pub(crate) fn new() -> std::io::Result<Self> {
        prepare_temp_frame_dir().map(|path| Self { path })
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempFrameDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub(crate) fn render_image_frame(
    temp_dir: &Path,
    frame_index: usize,
    image: &DynamicImage,
    width: u16,
    height: u16,
) -> Result<Vec<Line<'static>>, String> {
    render_image_frame_with_command("chafa", temp_dir, frame_index, image, width, height)
}

pub(crate) fn render_image_frame_with_command(
    command: &str,
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
    let rendered = render_frame_with_command(command, temp_path, width, height);
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::{
        cache_is_fresh_against, chafa_preset_args, chafa_version, decode_gif_frames,
        decode_gif_frames_with_delays, hero_frames, render_frame_with_command, HERO_RENDER_HEIGHT,
        HERO_RENDER_WIDTH,
    };
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
                0,
                "frame {frame_index} corner must stay transparent, not flattened to a matte"
            );
        }
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

    #[test]
    fn hero_cache_path_includes_the_renderer_revision() {
        let path = super::hero_frame_cache_path(4, 2);
        assert!(path.ends_with("hero_gif_1.r3.4x2.frame_cache.json"));
    }

    #[test]
    fn chafa_preset_args_is_stable_and_non_empty() {
        let args = chafa_preset_args();
        assert!(!args.is_empty());
        assert_eq!(args, chafa_preset_args(), "preset must be deterministic");
        assert!(args.contains(&"--symbols=braille".to_string()));
    }

    #[test]
    fn chafa_version_falls_back_to_unknown_for_missing_binary() {
        assert_eq!(chafa_version("__yam_missing_chafa_binary__"), "unknown");
    }

    #[test]
    fn decode_with_delays_matches_frame_count_of_plain_decode() {
        let plain = decode_gif_frames(super::HERO_GIF_PATH).expect("decode hero gif");
        let with_delays =
            decode_gif_frames_with_delays(super::HERO_GIF_PATH).expect("decode hero gif");
        assert_eq!(plain.len(), with_delays.len());
        assert!(
            with_delays.iter().all(|(_, delay_ms)| *delay_ms < 10_000),
            "hero frame delays should be small, not a units mixup"
        );
    }
}
