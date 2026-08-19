//! Hero source assets.
//!
//! A `HeroSource` is the single place that knows which GIF backs a hero, what
//! shape that GIF is, and how its frames are cached. Everything else in the
//! hero path (decode, chafa conversion, cache key, geometry tests) reads those
//! facts from here instead of repeating them, so adding or swapping hero art
//! is a descriptor change rather than a hunt through constants, cache-name
//! literals, tests, and docs.
//!
//! Render dimensions stay per-source because they are a chafa *request*, not a
//! guarantee: chafa preserves source aspect, so the emitted cell footprint is
//! derived from the frames themselves (see `render::hero`), not assumed here.

/// Stable identity for a hero asset.
///
/// The `stem` and `cache_revision` form the cache-file prefix, so two sources
/// cannot share frames and older renderer output cannot mask revised behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeroSource {
    /// Stable short id, also used as the frame-cache filename prefix.
    pub stem: &'static str,
    /// Absolute path to the source GIF, resolved at compile time.
    pub path: &'static str,
    /// Logical canvas the decoder expands every subimage frame onto.
    pub canvas_width: u32,
    pub canvas_height: u32,
    /// Frame count the asset is expected to carry.
    pub frame_count: usize,
    /// Cell footprint requested from chafa.
    pub render_width: u16,
    pub render_height: u16,
    /// Bump when source-specific compilation or the serialized frame contract
    /// changes, so an older cache can never mask the new renderer behavior.
    pub cache_revision: u16,
    /// Minimum percentage of the rendered grid frame 0 must still cover.
    ///
    /// Cell density is a property of the art, not of the pipeline: flat vector
    /// fills light fewer braille dots than textured cel shading at the same
    /// requested size, so one shared floor would either wave a real collapse
    /// through on the dense asset or fail the sparse one merely for being drawn
    /// differently. This is a collapse detector, so set it well under the
    /// asset's measured value rather than pinning the measurement itself.
    /// Whole percent keeps the descriptor `Eq` and the gate on integer math.
    pub min_frame0_coverage_percent: u8,
}

/// The original BTAS/TNBA-derived Ivy hero.
pub const IVY: HeroSource = HeroSource {
    stem: "hero_gif_1",
    path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif"),
    canvas_width: 820,
    canvas_height: 820,
    frame_count: 64,
    render_width: 96,
    render_height: 48,
    cache_revision: 2,
    // Measured 2026-08-19 against chafa 1.18.2: 932/4608 cells, 20.2%. The
    // floor is left at its original 20% rather than recalibrated here, but note
    // it now sits 0.2 points under the measurement, not the ~2x headroom the
    // gate's original note claimed. See docs/audit.md for the open decision.
    min_frame0_coverage_percent: 20,
};

/// The Moho vector rebuild of the same window loop.
///
/// Same character, same pose cycle, redrawn as flat vector art rather than
/// filtered from the raster original: `1080x1080` at 48 frames against `IVY`'s
/// `820x820` at 64. Registered as a second-source *probe*, not a hero swap:
/// `DEFAULT` stays `IVY`, so an ordinary launch is unaffected and only
/// `SOURCE_ENV` reaches this art. Its job is to make the descriptor seam and
/// its swap gates run against real second art instead of a registry of one.
pub const IVY_VECTOR: HeroSource = HeroSource {
    stem: "hero_gif_2",
    path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_2.gif"),
    canvas_width: 1080,
    canvas_height: 1080,
    frame_count: 48,
    render_width: 96,
    render_height: 48,
    cache_revision: 1,
    // Measured 2026-08-19 against chafa 1.18.2: 706/4608 cells, 15.3%. Lower
    // than IVY by art style, not by fault - flat fills and clean line work
    // simply light fewer dots than cel shading.
    min_frame0_coverage_percent: 10,
};

/// Every hero source the runtime knows about.
///
/// Read by `from_stem` on the runtime path and by the geometry, uniqueness,
/// and alpha gates in tests. `IVY_VECTOR` is a probe entry that proves the
/// registry works with more than one asset. The world/settings wiring that
/// would let something *persist* a choice between these is still the next
/// slice - `SOURCE_ENV` is a per-launch lever, not that surface - and this
/// registry is what that wiring will iterate.
pub const ALL: &[HeroSource] = &[IVY, IVY_VECTOR];

/// The source used when nothing selects one explicitly.
pub const DEFAULT: HeroSource = IVY;

/// Environment variable naming a registered source by `stem`.
///
/// This is the smallest honest form of the selection surface: enough to look
/// at candidate art in the running app before anything commits to it, without
/// inventing a settings-modal contract for a decision that has not been made.
pub const SOURCE_ENV: &str = "YAM_HERO_SOURCE";

/// The registered source with this `stem`, if any.
pub fn from_stem(stem: &str) -> Option<HeroSource> {
    ALL.iter().copied().find(|source| source.stem == stem)
}

/// Resolve a requested `stem` to a source, falling back to `DEFAULT`.
///
/// An unknown stem falls back rather than failing: a typo in an env var should
/// cost the reader their probe, not their hero.
pub fn resolve(requested: Option<&str>) -> HeroSource {
    requested.and_then(from_stem).unwrap_or(DEFAULT)
}

/// `resolve` against `SOURCE_ENV`.
pub fn resolve_from_env() -> HeroSource {
    match std::env::var(SOURCE_ENV) {
        Ok(stem) => resolve(Some(stem.trim())),
        Err(_) => DEFAULT,
    }
}

impl HeroSource {
    /// Frame-cache filename for this source at its render size.
    pub fn cache_file_name(&self) -> String {
        format!(
            "{}.r{}.{}x{}.frame_cache.json",
            self.stem, self.cache_revision, self.render_width, self.render_height
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{ALL, DEFAULT, IVY, IVY_VECTOR};

    #[test]
    fn cache_file_names_are_unique_per_source() {
        let mut names: Vec<String> = ALL.iter().map(|source| source.cache_file_name()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(
            names.len(),
            total,
            "each hero source needs its own frame cache file"
        );
    }

    #[test]
    fn source_stems_are_unique() {
        let mut stems: Vec<&str> = ALL.iter().map(|source| source.stem).collect();
        let total = stems.len();
        stems.sort();
        stems.dedup();
        assert_eq!(stems.len(), total, "hero source stems must be distinct");
    }

    #[test]
    fn default_source_is_registered() {
        assert!(ALL.contains(&DEFAULT), "default hero source must be in ALL");
    }

    #[test]
    fn ivy_cache_name_matches_the_documented_runtime_path() {
        assert_eq!(
            IVY.cache_file_name(),
            "hero_gif_1.r2.96x48.frame_cache.json"
        );
    }

    #[test]
    fn ivy_vector_cache_name_matches_the_documented_runtime_path() {
        assert_eq!(
            IVY_VECTOR.cache_file_name(),
            "hero_gif_2.r1.96x48.frame_cache.json"
        );
    }

    #[test]
    fn from_stem_finds_every_registered_source() {
        for source in ALL {
            assert_eq!(
                super::from_stem(source.stem),
                Some(*source),
                "{} should be reachable by stem",
                source.stem
            );
        }
    }

    #[test]
    fn resolve_falls_back_to_default_for_absent_or_unknown_stems() {
        assert_eq!(super::resolve(None), DEFAULT);
        assert_eq!(super::resolve(Some("no_such_hero")), DEFAULT);
        assert_eq!(super::resolve(Some("")), DEFAULT);
    }

    #[test]
    fn resolve_selects_the_probe_source_by_stem() {
        assert_eq!(super::resolve(Some(IVY_VECTOR.stem)), IVY_VECTOR);
        assert_ne!(super::resolve(Some(IVY_VECTOR.stem)), DEFAULT);
    }

    /// The probe only proves the seam if it is genuinely a different asset:
    /// same file or same declared geometry as `IVY` would make the geometry
    /// gate pass for the wrong reason.
    #[test]
    fn ivy_vector_is_a_distinct_asset_from_ivy() {
        assert_ne!(IVY_VECTOR.path, IVY.path);
        assert_ne!(
            (
                IVY_VECTOR.canvas_width,
                IVY_VECTOR.canvas_height,
                IVY_VECTOR.frame_count
            ),
            (IVY.canvas_width, IVY.canvas_height, IVY.frame_count)
        );
    }
}
