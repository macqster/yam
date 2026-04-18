# Monstera Leaf Study Notes

This file records the actionable outcomes from the external study at:

- `/Users/maciejkuster/Desktop/study_leaf_monstera_report.md`

It exists to preserve the design conclusions in the repo before large-leaf work begins.

## Status

This is a deferred design reference.

It is not an instruction to start large-leaf work immediately.

Current priority remains:

1. stabilize trunk route behavior
2. stabilize hero-mask boundary crawling
3. only then introduce large leaves

## Core Conclusion

The study's main result is structural:

> A believable terminal monstera leaf should be built from an upward-growing anchored spine and a lobe-driven outer shell, with edge cuts added only after the blade mass works.

That conclusion is compatible with the current `yam` direction and should be treated as the starting rule for future large-leaf work.

## Hard Rules For Future Leaf Work

- silhouette first
- structure before detail
- bottom-origin upward growth
- broad lower-middle blade mass
- edge-attached cuts only
- minimal role-based glyph vocabulary
- asymmetry only after shell stability
- lifecycle logic separate from morphology/rendering

## What To Avoid

The study repeatedly showed that these directions are weak in a terminal-grid context:

- hole-first design
- too many glyph classes too early
- oval-plus-cuts as a final model
- floating cuts
- decorative interior marks before shell stability
- over-dominant spine

## Recommended Future Architecture

When large leaves are added to `yam`, prefer a dedicated subsystem split into:

- leaf morphology generator
- glyph mapper / renderer
- lifecycle controller
- placement / attachment logic

Do not bolt complex leaf generation directly onto the current small ornament stamping path.

## Integration Guidance

When this work starts, the first implementation goal should be:

- one readable large leaf
- anchored to a meaningful vine/stem point
- no holes
- no rich fill
- no full lifecycle polish

Only after the shell reads correctly should later phases add:

- slight asymmetry
- limited edge cuts
- optional internal structure
- lifecycle stages

## Relation To Current Vines Work

This study does not change the current trunk-routing priority.

It clarifies sequencing:

- first solve vine/trunk behavior
- then use these notes to design large-leaf morphology
- then later revisit flowers and richer lifecycle ornament
