# YAM v2 Dependency Matrix

This file records which external terminal UI projects are available and how `yam` should treat them.

## Core Runtime

These are the default dependencies for the Bubble Tea path:

- [`charmbracelet/bubbletea`](https://github.com/charmbracelet/bubbletea)
- [`charmbracelet/lipgloss`](https://github.com/charmbracelet/lipgloss)
- [`charmbracelet/bubbles`](https://github.com/charmbracelet/bubbles)
- [`charmbracelet/vhs`](https://github.com/charmbracelet/vhs)
- [`common-nighthawk/go-figure`](https://github.com/common-nighthawk/go-figure)

## Optional Extensions

These are public and available, but should be adopted only when they solve a specific `yam` problem:

- [`charm-and-friends/additional-bubbles`](https://github.com/charm-and-friends/additional-bubbles)
- [`treilik/bubbleboxer`](https://github.com/treilik/bubbleboxer)
- [`mritd/bubbles`](https://github.com/mritd/bubbles)

## Reference Only

These can inform design or migration ideas, but should not be treated as direct dependencies without a deliberate decision:

- [`Genekkion/theHermit`](https://github.com/Genekkion/theHermit)

## Notes

- Prefer Charm-maintained libraries first.
- Use `go-figure` for FIGlet-based observer instrumentation when the scene needs ASCII typography.
- Keep third-party additions explicit in the roadmap and log before adoption.
- Do not expand the dependency set unless it improves a concrete `yam` capability.
- See [`LAUNCHER.md`](LAUNCHER.md) for how these categories map to the default runtime path and fallback path.
