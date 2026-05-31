# ChatGPT Usage Guide For YAM 0.4

## Goal

Use ChatGPT as a bounded planning partner, not as a substitute for the repo's
own contracts.

The best outputs are compact, option-oriented, and easy to promote back into
YAM docs.

## Good Request Shapes

Ask for:

- three to five room concepts with role, mood, fixtures, environment profile,
  and first organism fit
- support or fixture catalogs with concise tradeoffs
- naming passes for rooms, bays, supports, warnings, or inspection surfaces
- journal/inspection text tone samples
- comparison tables between two or three expansion shapes
- critique of a current plan against explicit YAM constraints
- compact implementation-order checks such as "what must exist before this"

## Bad Request Shapes

Avoid asking for:

- one giant complete design document
- implementation-ready Rust architecture invented from scratch
- dashboard UI proposals without world-space reasoning
- broad gameplay loops before greenhouse ownership is proven
- persistence/database plans before the first inert room exists
- repo-audit conclusions from the source pack alone

## Prompt Rules

When prompting ChatGPT, explicitly include:

- YAM is a Rust Ratatui terminal project
- greenhouse is a place, not an admin dashboard
- render layers are read-only visualizers
- first greenhouse pass is functional-space-first
- first room is `greenhouse_nursery`
- room/support/organism identities must stay distinct
- output should be candidate planning material, not implementation authority
- if the pack is partial, say so explicitly and name what is missing

## Recommended Prompt Frame

For high-value external sessions, frame the request like this:

1. what YAM is
2. what phase we are actually in
3. what not to propose yet
4. what shape of answer is wanted
5. how many options or variants to return

## Preferred Output Style

Ask for outputs that are:

- concise
- structured
- option-based
- vocabulary-rich
- compatible with terminal rendering
- explicit about risks and tradeoffs
- aware of first-pass versus later-phase distinctions

## Promotion Rule

Any useful ChatGPT output should be promoted carefully:

1. distill it into a YAM-owned doc
2. align it with roadmap and backlog
3. keep only the parts that respect the current architecture
4. avoid implementing from raw transcript text

## Good Failure Mode

If ChatGPT is missing repo context, the good answer is a scoped assumption plus
a bounded suggestion set, not fake certainty.
