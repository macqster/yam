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
};
use crate::render::hero_manifest::{HeroManifest, LoopMode, HERO_PACKAGE_SCHEMA_REVISION};
use crate::render::hero_package::{save_hero_package, HeroPackage};
use crate::render::hero_source::{self, HeroSource};

#[derive(Clone, Debug)]
pub struct CompileOptions {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub render_width: u16,
    pub render_height: u16,
    pub chafa_command: String,
    /// The colour chafa is told is already on screen. Carried per compile so
    /// the offline package is rendered against the same drop reference as the
    /// runtime, rather than a constant that could drift from the descriptor.
    pub absent_color: [u8; 3],
}

impl CompileOptions {
    /// Compile a registered hero source, taking geometry and drop reference
    /// from its descriptor so an offline package cannot be rendered against
    /// different settings than the runtime uses for the same asset.
    pub fn for_source(source: &HeroSource) -> Self {
        Self {
            source_path: PathBuf::from(source.path),
            output_path: chafa::hero_package_path(source),
            render_width: source.render_width,
            render_height: source.render_height,
            chafa_command: "chafa".to_string(),
            absent_color: source.absent_color,
        }
    }
}

impl Default for CompileOptions {
    /// The source an ordinary launch would render, so `--compile-hero` and the
    /// runtime agree by default. `YAM_HERO_SOURCE` selects here too.
    fn default() -> Self {
        Self::for_source(&hero_source::resolve_from_env())
    }
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
            options.absent_color,
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
        compiler_args: chafa::chafa_preset_args(options.absent_color),
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
    use crate::render::hero_source;
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

    /// The compiler must follow the registry rather than a hard-coded asset.
    /// Pinning a filename here is what made this test fail when the default
    /// hero moved to `hero_gif_2` - the compiler would have kept producing
    /// packages for whichever asset happened to be named first.
    #[test]
    fn default_options_track_the_default_hero_source() {
        let options = CompileOptions::default();
        let default_source = hero_source::DEFAULT;
        assert_eq!(options.source_path, PathBuf::from(default_source.path));
        assert_eq!(options.render_width, default_source.render_width);
        assert_eq!(options.render_height, default_source.render_height);
        assert_eq!(options.absent_color, default_source.absent_color);
        assert_eq!(options.chafa_command, "chafa");
    }

    /// Every registered source must be compilable, and each must carry its own
    /// geometry and drop reference - a package rendered against another
    /// source's `absent_color` would silently contain the wrong art.
    #[test]
    fn options_for_each_source_carry_that_source_geometry_and_drop_reference() {
        for source in hero_source::ALL {
            let options = CompileOptions::for_source(source);
            assert_eq!(options.source_path, PathBuf::from(source.path));
            assert_eq!(options.render_width, source.render_width);
            assert_eq!(options.render_height, source.render_height);
            assert_eq!(options.absent_color, source.absent_color);
        }
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
