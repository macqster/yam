//! The validated, versioned hero package: the "compiled package" owner
//! layer in docs/hero-revision.md's pipeline table.
//!
//! `HeroPackage` is distinct from `render::hero_cache::HeroFrameSet` on
//! purpose: the cache is disposable runtime acceleration that may vanish or
//! go stale at any time, while a `HeroPackage` is the intentional,
//! reviewable artifact produced by the offline compiler
//! (`render::hero_compiler`, invoked as `yam-rust --compile-hero`) and is
//! expected to be inspected, versioned, and validated before anything
//! treats it as correct.
//!
//! `validate()` only checks objective, machine-checkable facts (frame
//! count, fixed geometry, non-blank frames, provenance completeness). It
//! deliberately does not and cannot judge color fidelity -- per
//! docs/hero-revision.md, "ANSI-code presence and non-placeholder frame
//! counts are insufficient" and a real-terminal review via
//! `scripts/tmux-smoke.sh` is still required after any compiler/source
//! change.

use std::{fmt, fs, io, path::Path};

use serde::{Deserialize, Serialize};

use crate::render::cell_grid::CellGrid;
use crate::render::hero_manifest::{HeroManifest, HERO_PACKAGE_SCHEMA_REVISION};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeroPackage {
    pub manifest: HeroManifest,
    pub frames: Vec<CellGrid>,
}

/// The result of `HeroPackage::validate()`: a list of concrete, named
/// problems rather than a single opaque failure, so a compiler run can
/// report everything wrong in one pass instead of one issue at a time.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PackageValidation {
    pub issues: Vec<String>,
}

impl PackageValidation {
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }
}

impl fmt::Display for PackageValidation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.issues.is_empty() {
            write!(f, "package valid: no issues found")
        } else {
            writeln!(f, "package invalid: {} issue(s)", self.issues.len())?;
            for issue in &self.issues {
                writeln!(f, "  - {issue}")?;
            }
            Ok(())
        }
    }
}

impl HeroPackage {
    pub fn validate(&self) -> PackageValidation {
        let mut issues = Vec::new();
        let manifest = &self.manifest;

        if self.frames.len() != manifest.frame_count {
            issues.push(format!(
                "frame_count mismatch: manifest says {}, package has {} frames",
                manifest.frame_count,
                self.frames.len()
            ));
        }

        if manifest.frame_durations_ms.len() != manifest.frame_count {
            issues.push(format!(
                "frame_durations_ms length ({}) does not match frame_count ({})",
                manifest.frame_durations_ms.len(),
                manifest.frame_count
            ));
        }

        if self.frames.is_empty() {
            issues.push("package has zero frames".to_string());
        }

        for (index, frame) in self.frames.iter().enumerate() {
            let expected_cells = frame.width as usize * frame.height as usize;
            if frame.cells.len() != expected_cells {
                issues.push(format!(
                    "frame {index} cell count {} does not match geometry {}x{}",
                    frame.cells.len(),
                    frame.width,
                    frame.height
                ));
            }
            if frame.width != manifest.render_width || frame.height != manifest.render_height {
                issues.push(format!(
                    "frame {index} geometry {}x{} does not match manifest render size {}x{}",
                    frame.width, frame.height, manifest.render_width, manifest.render_height
                ));
            }
            if frame.is_blank() {
                issues.push(format!(
                    "frame {index} is entirely blank (unstyled spaces); \
                     likely a placeholder or a failed compile step"
                ));
            }
        }

        if manifest.compiler_id.trim().is_empty() {
            issues.push("manifest.compiler_id is empty".to_string());
        }
        if manifest.preset_id.trim().is_empty() {
            issues.push("manifest.preset_id is empty".to_string());
        }
        if manifest.asset_digest.trim().is_empty() {
            issues.push("manifest.asset_digest is empty".to_string());
        }
        if manifest.schema_revision != HERO_PACKAGE_SCHEMA_REVISION {
            issues.push(format!(
                "unsupported schema revision {} (expected {})",
                manifest.schema_revision, HERO_PACKAGE_SCHEMA_REVISION
            ));
        }

        PackageValidation { issues }
    }
}

#[allow(dead_code)]
pub fn load_hero_package(path: &Path) -> io::Result<HeroPackage> {
    let json = fs::read_to_string(path)?;
    serde_json::from_str(&json).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

pub fn save_hero_package(path: &Path, package: &HeroPackage) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string(package)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    fs::write(path, json)
}

#[cfg(test)]
mod tests {
    use super::{HeroPackage, PackageValidation};
    use crate::render::cell_grid::CellGrid;
    use crate::render::hero_manifest::{HeroManifest, LoopMode, HERO_PACKAGE_SCHEMA_REVISION};
    use ratatui::style::{Color, Style};
    use ratatui::text::{Line, Span};
    use tempfile::tempdir;

    fn styled_frame(width: u16, height: u16) -> CellGrid {
        let lines = vec![
            Line::from(vec![Span::styled(
                "x".repeat(width as usize),
                Style::default().fg(Color::Rgb(114, 22, 15)),
            )]);
            height as usize
        ];
        CellGrid::from_lines(&lines, width, height)
    }

    fn manifest(frame_count: usize) -> HeroManifest {
        HeroManifest {
            asset_id: "hero_gif_1".to_string(),
            asset_digest: "deadbeef".to_string(),
            canvas_width: 4,
            canvas_height: 2,
            frame_count,
            frame_durations_ms: vec![40; frame_count],
            loop_mode: LoopMode::Infinite,
            compiler_id: "chafa".to_string(),
            compiler_version: "1.14.0".to_string(),
            preset_id: "rgb-median-fgonly-braille-v1".to_string(),
            compiler_args: vec!["--symbols=braille".to_string()],
            render_width: 2,
            render_height: 1,
            schema_revision: HERO_PACKAGE_SCHEMA_REVISION,
        }
    }

    #[test]
    fn well_formed_package_validates_clean() {
        let package = HeroPackage {
            manifest: manifest(2),
            frames: vec![styled_frame(2, 1), styled_frame(2, 1)],
        };
        let report = package.validate();
        assert!(report.is_valid(), "expected no issues, got {report}");
    }

    #[test]
    fn frame_count_mismatch_is_caught() {
        let package = HeroPackage {
            manifest: manifest(3),
            frames: vec![styled_frame(2, 1), styled_frame(2, 1)],
        };
        let report = package.validate();
        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|i| i.contains("frame_count")));
    }

    #[test]
    fn mismatched_frame_geometry_is_caught() {
        let package = HeroPackage {
            manifest: manifest(1),
            frames: vec![styled_frame(3, 1)],
        };
        let report = package.validate();
        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|i| i.contains("geometry")));
    }

    #[test]
    fn blank_frame_is_caught() {
        let package = HeroPackage {
            manifest: manifest(1),
            frames: vec![CellGrid::from_lines(&[], 2, 1)],
        };
        let report = package.validate();
        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|i| i.contains("blank")));
    }

    #[test]
    fn truncated_cell_storage_is_caught() {
        let mut frame = styled_frame(2, 1);
        frame.cells.pop();
        let package = HeroPackage {
            manifest: manifest(1),
            frames: vec![frame],
        };
        let report = package.validate();
        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|i| i.contains("cell count")));
    }

    #[test]
    fn unsupported_schema_revision_is_caught() {
        let mut m = manifest(1);
        m.schema_revision = HERO_PACKAGE_SCHEMA_REVISION + 1;
        let package = HeroPackage {
            manifest: m,
            frames: vec![styled_frame(2, 1)],
        };
        let report = package.validate();
        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|i| i.contains("schema revision")));
    }

    #[test]
    fn empty_compiler_id_is_caught() {
        let mut m = manifest(1);
        m.compiler_id = String::new();
        let package = HeroPackage {
            manifest: m,
            frames: vec![styled_frame(2, 1)],
        };
        let report = package.validate();
        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|i| i.contains("compiler_id")));
    }

    #[test]
    fn empty_validation_report_displays_as_valid() {
        let report = PackageValidation::default();
        assert!(report.to_string().contains("valid"));
    }

    #[test]
    fn hero_package_round_trips_through_disk() {
        let package = HeroPackage {
            manifest: manifest(1),
            frames: vec![styled_frame(2, 1)],
        };
        let dir = tempdir().expect("temp dir should exist");
        let path = dir.path().join("hero-package.json");

        super::save_hero_package(&path, &package).expect("package should save");
        let loaded = super::load_hero_package(&path).expect("package should load");

        assert_eq!(loaded, package);
    }
}
