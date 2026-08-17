//! The hero compiler-preset/provenance contract.
//!
//! `HeroManifest` is the explicit record docs/hero-revision.md's Phase 1
//! calls for: it names every transform and tool version that affects a
//! compiled hero package's output, so a package can be traced back to the
//! exact source asset and compiler invocation that produced it. This type
//! carries no rendering logic; it is pure provenance data.

use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Bumped whenever `HeroManifest` or `HeroPackage`'s on-disk shape changes
/// in a way older readers cannot tolerate. Distinct from
/// `hero_cache::HERO_CACHE_REVISION` in `render/chafa.rs`, which governs the
/// disposable runtime cache, not this validated package format.
pub const HERO_PACKAGE_SCHEMA_REVISION: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopMode {
    Infinite,
    Once,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeroManifest {
    /// Stable logical name for the source asset, independent of its file
    /// path, e.g. `"hero_gif_1"`.
    pub asset_id: String,
    /// A stable SHA-256 digest of the source GIF's raw bytes, hex-encoded.
    /// This identifies which exact source bytes produced this package.
    pub asset_digest: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub frame_count: usize,
    /// Per-frame delay in milliseconds, in source-GIF order. Length must
    /// equal `frame_count`.
    pub frame_durations_ms: Vec<u32>,
    pub loop_mode: LoopMode,
    /// The compiler backend that produced this package, e.g. `"chafa"`. A
    /// future custom braille backend gets its own id here rather than
    /// overloading this one.
    pub compiler_id: String,
    /// Best-effort captured version string of the compiler binary at
    /// compile time (e.g. `chafa --version`'s first line), or `"unknown"`
    /// if it could not be captured.
    pub compiler_version: String,
    /// A short human-readable name for the exact preset used, e.g.
    /// `"rgb-median-fgonly-braille-v1"`. Change this whenever the flags
    /// below change in a way that affects visible output.
    pub preset_id: String,
    /// The literal compiler arguments used, in order, excluding the input
    /// path and `--size`. Kept alongside `preset_id` so a package's exact
    /// invocation is reconstructable without cross-referencing source code.
    pub compiler_args: Vec<String>,
    pub render_width: u16,
    pub render_height: u16,
    pub schema_revision: u16,
}

impl HeroManifest {
    /// Computes `asset_digest` from a source file's raw bytes.
    pub fn digest_source_file(path: &Path) -> io::Result<String> {
        let bytes = fs::read(path)?;
        Ok(digest_bytes(&bytes))
    }
}

fn digest_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::digest_bytes;

    #[test]
    fn digest_is_deterministic_for_identical_bytes() {
        assert_eq!(
            digest_bytes(b"hero frame data"),
            digest_bytes(b"hero frame data")
        );
    }

    #[test]
    fn digest_differs_for_different_bytes() {
        assert_ne!(digest_bytes(b"frame a"), digest_bytes(b"frame b"));
    }

    #[test]
    fn digest_uses_the_stable_sha256_hex_shape() {
        assert_eq!(digest_bytes(b"hero frame data").len(), 64);
    }
}
