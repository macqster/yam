# YAM v2 FIGlet Tooling Review

This note tracks the FIGlet libraries evaluated for the clock typography path.

## Current Decision

- keep `v2/render/fonts/go_deco.txt` as the canonical clock font asset
- keep the Go Bubble Tea runtime as the live renderer
- keep Python as a thin snapshot and golden-frame helper only

## Libraries Reviewed

- `github.com/common-nighthawk/go-figure`
- `github.com/mattn/go-figlet`
- `github.com/mbndr/figlet4go`
- `github.com/lsferreira42/figlet-go`

## Notes

- `go-figure` worked for basic FIGlet output, but its spacing behavior was not a good fit for the clock scene.
- `figlet4go` is a long-running FIGlet port, but it is an older codebase.
- `lsferreira42/figlet-go` is the most interesting future candidate if we want a fuller FIGlet-style API again.
- none of the FIGlet libraries are needed for the current canonical clock renderer.

## Recommendation

- keep the native renderer for now
- revisit `lsferreira42/figlet-go` only if the YAM typography direction changes back toward FIGlet rendering
- do not reintroduce a second clock implementation without a tracked reason
