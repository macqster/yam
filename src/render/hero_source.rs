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
    /// A colour that appears nowhere in this asset.
    ///
    /// Passed to chafa as `--bg`, which is a badly misleading name for what it
    /// does under `--fg-only`: chafa never paints it, and it is not a claim
    /// about what the terminal looks like. It is the colour chafa treats as
    /// *already on screen*, so any art resembling it is judged redundant and
    /// silently dropped rather than drawn.
    ///
    /// That is why this must be absent from the art. Pointing it at a
    /// scene-like dark colour - as `HERO_DISPLAY_BG` did until 0.4.2 - tells
    /// chafa that every dark region is already painted, which is exactly how
    /// the hair shadow, the leggings, and every outline went missing.
    ///
    /// Distance is a trade rather than something to push as far as possible:
    /// too close and art is dropped, but on partially transparent edge cells
    /// this colour also bleeds into chafa's foreground pick, and that bleed
    /// grows with distance. Aim for the least clearance that still clears the
    /// drop radius. `absent_color_is_actually_absent_from_every_source` keeps the
    /// near side honest.
    pub absent_color: [u8; 3],
}

/// The original BTAS/TNBA-derived Ivy hero.
///
/// The default until 0.4.1, when `IVY_VECTOR` replaced it. Still registered
/// and still gated, so it stays reachable through `SOURCE_ENV` and keeps its
/// own frame cache rather than becoming unreferenced art.
pub const IVY: HeroSource = HeroSource {
    stem: "hero_gif_1",
    path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif"),
    canvas_width: 820,
    canvas_height: 820,
    frame_count: 64,
    render_width: 96,
    render_height: 48,
    cache_revision: 5,
    // Measured 2026-08-19 against chafa 1.18.2: 932/4608 cells, 20.2%. The
    // floor is left at its original 20% rather than recalibrated here, but note
    // it now sits 0.2 points under the measurement, not the ~2x headroom the
    // gate's original note claimed. See docs/audit.md for the open decision.
    min_frame0_coverage_percent: 20,
    // Nearest renderable art colour is the green eye, (70, 78, 4), at 162.
    absent_color: [0, 224, 0],
};

/// The Moho vector rebuild of the same window loop, and the hero since 0.4.1.
///
/// Same character, same pose cycle, redrawn as flat vector art rather than
/// filtered from the raster original: `1080x1080` at 48 frames against `IVY`'s
/// `820x820` at 64. It entered the registry as a probe and was promoted to
/// `DEFAULT` once it had been looked at in the running app.
pub const IVY_VECTOR: HeroSource = HeroSource {
    stem: "hero_gif_2",
    path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_2.gif"),
    canvas_width: 1080,
    canvas_height: 1080,
    frame_count: 48,
    render_width: 96,
    render_height: 48,
    cache_revision: 4,
    // Measured 2026-08-19 against chafa 1.18.2 at the chosen absent_color:
    // 923/4608 cells, 20.0%.
    min_frame0_coverage_percent: 10,
    // #336699. Unlike IVY this deliberately overlaps its own palette, because
    // the cull is wanted: flat fills render as fully-lit braille, and dropping
    // the darkest tiers is what keeps the hero open rather than a solid mass.
    // Chosen because 7c0307 and 332a29 have identical RGB sums (134 each), so
    // no neutral value separates them - a chromatically opposite one does,
    // keeping the dark red while still dropping the leggings and line art.
    // Nearest art colour (104, 90, 110) at 69; see ACCEPTED_OVERLAP in chafa.rs.
    absent_color: [51, 102, 153],
};

/// Every hero source the runtime knows about.
///
/// Read by `from_stem` on the runtime path and by the geometry, uniqueness,
/// and alpha gates in tests. Both entries are live art: `IVY_VECTOR` renders
/// by default and `IVY` remains reachable, so a swap is reversible without a
/// rebuild. The world/settings wiring that would let something *persist* a
/// choice between them is still the next slice - `SOURCE_ENV` is a per-launch
/// lever, not that surface - and this registry is what that wiring will
/// iterate.
pub const ALL: &[HeroSource] = &[IVY, IVY_VECTOR];

/// The source used when nothing selects one explicitly.
pub const DEFAULT: HeroSource = IVY_VECTOR;

/// Environment variable naming a registered source by `stem`.
///
/// This is the smallest honest form of the selection surface: enough to look
/// at candidate art in the running app before anything commits to it, and now
/// also the way back to `IVY` (`YAM_HERO_SOURCE=hero_gif_1`) without a
/// rebuild. It is deliberately not a settings-modal contract.
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

    /// Which asset an ordinary launch renders is a product decision, not an
    /// incidental ordering of `ALL`. Pin it so a swap has to be deliberate and
    /// arrives with the docs that name the same stem.
    #[test]
    fn default_source_is_the_vector_hero() {
        assert_eq!(DEFAULT.stem, "hero_gif_2");
        assert_eq!(DEFAULT, IVY_VECTOR);
    }

    #[test]
    fn ivy_cache_name_matches_the_documented_runtime_path() {
        assert_eq!(
            IVY.cache_file_name(),
            "hero_gif_1.r5.96x48.frame_cache.json"
        );
    }

    #[test]
    fn ivy_vector_cache_name_matches_the_documented_runtime_path() {
        assert_eq!(
            IVY_VECTOR.cache_file_name(),
            "hero_gif_2.r4.96x48.frame_cache.json"
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

    /// Selecting by stem has to actually return that source. Silently falling
    /// back to `DEFAULT` is the failure mode this path can hide behind - it
    /// looks exactly like "the variable did nothing" - so prove it against a
    /// source that is deliberately not the default, whichever that currently
    /// is.
    #[test]
    fn resolve_selects_a_non_default_source_by_stem() {
        let other = ALL
            .iter()
            .find(|source| **source != DEFAULT)
            .expect("registry needs a non-default source for this to prove anything");
        assert_eq!(super::resolve(Some(other.stem)), *other);
        assert_ne!(super::resolve(Some(other.stem)), DEFAULT);
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
