//! Offline hero package compiler.
//!
//! This is the "explicit offline compiler command" docs/hero-revision.md's
//! Phase 1 calls for: it decodes the source GIF, compiles each frame
//! through the same chafa preset the runtime uses, and emits a validated,
//! versioned `HeroPackage` -- without ever entering the interactive
//! runtime. Invoked via `yam-rust --compile-hero` (see `src/main.rs`);
//! never runs as part of ordinary startup.
//!
//! What this module's report does *not* prove: color fidelity. Per
//! docs/hero-revision.md, "ANSI-code presence and non-placeholder frame
//! counts are insufficient." `HeroPackage::validate()` only checks
//! objective, machine-checkable facts. A real-terminal review via
//! `scripts/tmux-smoke.sh` is still required before trusting output from
//! this compiler.

use std::path::{Path, PathBuf};

use crate::render::cell_grid::CellGrid;
use crate::render::chafa::{
    self, decode_gif_frames_with_delays, render_image_frame_with_command, TempFrameDir,
    HERO_GIF_PATH, HERO_RENDER_HEIGHT, HERO_RENDER_WIDTH,
};
use crate::render::hero_manifest::{HeroManifest, LoopMode, HERO_PACKAGE_SCHEMA_REVISION};
use crate::render::hero_package::{save_hero_package, HeroPackage};

#[derive(Clone, Debug)]
pub struct CompileOptions {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub render_width: u16,
    pub render_height: u16,
    pub chafa_command: String,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            source_path: PathBuf::from(HERO_GIF_PATH),
            output_path: default_output_path(),
            render_width: HERO_RENDER_WIDTH,
            render_height: HERO_RENDER_HEIGHT,
            chafa_command: "chafa".to_string(),
        }
    }
}

pub fn default_output_path() -> PathBuf {
    PathBuf::from("target/hero_package.json")
}

/// Runs the full compile: decode -> per-frame chafa render -> manifest ->
/// validate -> write. Prints a human-readable validation report and a
/// same-caveat reminder to stdout; returns `Err` (with the report already
/// printed) if validation fails, so a failed compile cannot silently
/// produce a package a future run would trust.
pub fn compile(options: &CompileOptions) -> Result<HeroPackage, String> {
    let source_str = options
        .source_path
        .to_str()
        .ok_or_else(|| format!("source path not utf-8: {:?}", options.source_path))?;

    let decoded = decode_gif_frames_with_delays(source_str)?;
    if decoded.is_empty() {
        return Err(format!("no frames decoded from {source_str}"));
    }
    let (first_image, _) = &decoded[0];
    let canvas_width = first_image.width();
    let canvas_height = first_image.height();
    let frame_durations_ms: Vec<u32> = decoded.iter().map(|(_, delay_ms)| *delay_ms).collect();

    let temp_dir =
        TempFrameDir::new().map_err(|err| format!("failed to create temp frame dir: {err}"))?;

    let mut frames = Vec::with_capacity(decoded.len());
    for (index, (image, _)) in decoded.iter().enumerate() {
        let lines = render_image_frame_with_command(
            &options.chafa_command,
            temp_dir.path(),
            index,
            image,
            options.render_width,
            options.render_height,
        )
        .map_err(|err| format!("frame {index} render failed: {err}"))?;
        frames.push(CellGrid::from_lines(
            &lines,
            options.render_width,
            options.render_height,
        ));
    }

    let asset_digest = HeroManifest::digest_source_file(&options.source_path)
        .map_err(|err| format!("failed to digest {source_str}: {err}"))?;

    let manifest = HeroManifest {
        asset_id: asset_id_from_path(&options.source_path),
        asset_digest,
        canvas_width,
        canvas_height,
        frame_count: frames.len(),
        frame_durations_ms,
        loop_mode: LoopMode::Infinite,
        compiler_id: "chafa".to_string(),
        compiler_version: chafa::chafa_version(&options.chafa_command),
        preset_id: chafa::HERO_PRESET_ID.to_string(),
        compiler_args: chafa::chafa_preset_args(),
        render_width: options.render_width,
        render_height: options.render_height,
        schema_revision: HERO_PACKAGE_SCHEMA_REVISION,
    };

    let package = HeroPackage { manifest, frames };
    let report = package.validate();
    println!("{report}");
    if !report.is_valid() {
        return Err(format!(
            "compiled package failed validation ({} issue(s)); see report above",
            report.issues.len()
        ));
    }

    save_hero_package(&options.output_path, &package).map_err(|err| {
        format!(
            "failed to write package to {:?}: {err}",
            options.output_path
        )
    })?;

    println!(
        "wrote {} frames ({}x{} cells) to {:?}",
        package.frames.len(),
        options.render_width,
        options.render_height,
        options.output_path
    );
    println!(
        "NOTE: this report only checks frame count, geometry, and non-blank frames -- it \
         cannot judge color fidelity. Run scripts/tmux-smoke.sh and review a real terminal \
         per docs/hero-revision.md before trusting this package's dark-color output."
    );

    Ok(package)
}

fn asset_id_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("hero")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{asset_id_from_path, CompileOptions};
    use std::path::PathBuf;

    #[test]
    fn asset_id_uses_the_file_stem() {
        assert_eq!(
            asset_id_from_path(&PathBuf::from("/some/path/hero_gif_1.gif")),
            "hero_gif_1"
        );
    }

    #[test]
    fn asset_id_falls_back_when_stem_is_missing() {
        assert_eq!(asset_id_from_path(&PathBuf::from("/")), "hero");
    }

    #[test]
    fn default_options_point_at_the_canonical_hero_gif() {
        let options = CompileOptions::default();
        assert!(options
            .source_path
            .to_string_lossy()
            .ends_with("assets/hero_gif_1.gif"));
        assert_eq!(options.chafa_command, "chafa");
    }

    #[test]
    fn compile_reports_a_clear_error_for_a_missing_source() {
        let options = CompileOptions {
            source_path: PathBuf::from("__yam_missing_hero_source.gif"),
            ..CompileOptions::default()
        };
        let err = super::compile(&options).expect_err("missing source should fail");
        assert!(err.contains("__yam_missing_hero_source.gif"));
    }
}
