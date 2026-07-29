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
use crate::render::hero_source::HeroSource;

const HERO_DISPLAY_BG: Rgba<u8> = Rgba([16, 1, 0, 255]);

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
        .arg("--color-extractor=median")
        .arg("--dither=none")
        .arg("--fg-only")
        .arg(format!(
            "--bg=#{:02x}{:02x}{:02x}",
            HERO_DISPLAY_BG[0], HERO_DISPLAY_BG[1], HERO_DISPLAY_BG[2]
        ))
        .arg("--animate=off")
        .output()
}

pub fn hero_frames_from(source: &HeroSource, width: u16, height: u16) -> Vec<Vec<Line<'static>>> {
    let frames = match decode_gif_frames(source.path) {
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

pub fn hero_frames_cached_from(
    source: &HeroSource,
    width: u16,
    height: u16,
) -> Vec<Vec<Line<'static>>> {
    let cache_path = hero_frame_cache_path(source);
    if let Some(frame_set) = load_cached_hero_frames(&cache_path, source, width, height) {
        return frame_set.to_lines();
    }

    let frames = hero_frames_from(source, width, height);
    if hero_frames_are_cacheable(&frames) {
        let frame_set = HeroFrameSet::from_lines(width, height, &frames);
        let _ = save_hero_frame_set(&cache_path, &frame_set);
    }
    frames
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

fn load_cached_hero_frames(
    path: &Path,
    source: &HeroSource,
    width: u16,
    height: u16,
) -> Option<HeroFrameSet> {
    if !cache_is_fresh_against(path, Path::new(source.path)) {
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

fn hero_frame_cache_path(source: &HeroSource) -> PathBuf {
    hero_cache_dir().join(source.cache_file_name())
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

#[cfg(test)]
mod tests {
    use super::{
        cache_is_fresh_against, decode_gif_frames, hero_frames_from, render_frame_with_command,
    };
    use crate::render::hero_source::{self, HeroSource, DEFAULT as DEFAULT_HERO_SOURCE};
    use ratatui::text::Line;
    use std::{fs, thread, time::Duration};

    #[test]
    fn hero_frame_buffer_has_multiple_frames() {
        let frames = hero_frames_from(
            &DEFAULT_HERO_SOURCE,
            DEFAULT_HERO_SOURCE.render_width,
            DEFAULT_HERO_SOURCE.render_height,
        );
        assert!(frames.len() > 1, "expected multiple hero frames");
    }

    /// Is a real `chafa` available to this process?
    ///
    /// Content-level hero assertions are only meaningful when it is. CI does
    /// not install `chafa`, so every live-path render there legitimately
    /// yields placeholder frames; a content test that ignored this would fail
    /// in CI permanently rather than catch anything.
    fn chafa_is_available() -> bool {
        std::process::Command::new("chafa")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Rendered hero frames must contain actual art, not a batch of
    /// placeholders.
    ///
    /// This is the gate that was missing when an incomplete `image` feature
    /// trim silently broke every hero frame for a day in July 2026: the only
    /// live-path test checked frame *count*, which a placeholder batch
    /// satisfies just as well as real output. Skips rather than fails where
    /// `chafa` is absent, so it stays honest in CI instead of red.
    #[test]
    fn rendered_hero_frames_contain_real_content_not_placeholders() {
        if !chafa_is_available() {
            eprintln!("skipping: chafa not available, live render yields placeholders by design");
            return;
        }

        let source = &DEFAULT_HERO_SOURCE;
        let frames = hero_frames_from(source, source.render_width, source.render_height);
        assert_eq!(
            frames.len(),
            source.frame_count,
            "live render should produce every declared frame"
        );

        for (frame_index, frame) in frames.iter().enumerate() {
            assert!(
                !super::is_placeholder_frame(frame),
                "frame {frame_index} rendered as a placeholder: {:?}",
                frame_text(frame)
            );

            let covered = covered_cells(frame);
            assert!(
                covered > 0,
                "frame {frame_index} rendered no visible cells at all"
            );
        }

        // Coverage is the number the 2026-07-22 investigation moved: the
        // pre-fix pipeline dropped roughly 80% of the grid as "no coverage".
        // Frame 0 measured 41.5% after that fix. A floor well under the
        // measurement catches a collapse without pinning an exact value that
        // legitimate art or extractor changes would churn.
        let first_covered = covered_cells(&frames[0]);
        let total = (source.render_width as usize) * (source.render_height as usize);
        let ratio = first_covered as f64 / total as f64;
        assert!(
            ratio > 0.20,
            "frame 0 coverage collapsed to {:.1}% of the grid ({first_covered}/{total})",
            ratio * 100.0
        );
    }

    fn covered_cells(frame: &[Line<'static>]) -> usize {
        frame
            .iter()
            .flat_map(|line| line.spans.iter())
            .flat_map(|span| span.content.chars())
            .filter(|ch| !ch.is_whitespace() && *ch != '\u{2800}')
            .count()
    }

    fn frame_text(frame: &[Line<'static>]) -> String {
        frame
            .iter()
            .flat_map(|line| line.spans.iter())
            .map(|span| span.content.as_ref())
            .collect::<String>()
            .chars()
            .take(80)
            .collect()
    }

    /// Every registered hero source must decode to the frame count and canvas
    /// geometry its descriptor claims. This is the swap gate: new art fails
    /// here first if its descriptor is wrong, instead of silently rendering
    /// at the wrong size.
    #[test]
    fn every_hero_source_matches_its_declared_geometry() {
        for source in hero_source::ALL {
            assert_declared_geometry(source);
        }
    }

    fn assert_declared_geometry(source: &HeroSource) {
        let stem = source.stem;
        let frames = decode_gif_frames(source.path)
            .unwrap_or_else(|err| panic!("decode hero gif {stem}: {err}"));
        assert_eq!(
            frames.len(),
            source.frame_count,
            "{stem} declared {} frames",
            source.frame_count
        );

        for (frame_index, frame) in frames.iter().enumerate() {
            assert_eq!(
                frame.width(),
                source.canvas_width,
                "{stem} frame {frame_index} width"
            );
            assert_eq!(
                frame.height(),
                source.canvas_height,
                "{stem} frame {frame_index} height"
            );
        }
    }

    /// The alpha contract: subimage frames are expanded onto a transparent
    /// canvas, never flattened onto an opaque matte. Losing this is what cost
    /// the renderer its dark regions before 2026-07-22.
    #[test]
    fn hero_frames_keep_a_transparent_canvas_rather_than_a_matte() {
        for source in hero_source::ALL {
            let stem = source.stem;
            let frames = decode_gif_frames(source.path)
                .unwrap_or_else(|err| panic!("decode hero gif {stem}: {err}"));
            for (frame_index, frame) in frames.iter().enumerate() {
                assert_eq!(
                    frame.to_rgba8().get_pixel(0, 0)[3],
                    0,
                    "{stem} frame {frame_index} corner must stay transparent, not flattened to a matte"
                );
            }
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
        let lines = render_frame_with_command(
            "__yam_missing_chafa_binary__",
            DEFAULT_HERO_SOURCE.path,
            4,
            2,
        );
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
