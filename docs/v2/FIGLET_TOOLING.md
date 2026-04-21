# YAM v2 FIGlet Tooling Review

This note tracks the FIGlet libraries evaluated for the clock typography path.

## Current Decision

- use `github.com/lsferreira42/figlet-go/figlet` for the live clock renderer
- use `Fender.flf` from `v2/render/fonts/` as the default live clock font
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
- `lsferreira42/figlet-go` is now the live FIGlet engine used by the Go runtime.
- `Fender.flf` is the current repo-local live clock font.
- the custom grid renderer and its font-file contract are historical context only.

## Recommendation

- keep `lsferreira42/figlet-go` as the live FIGlet engine unless a measured regression appears
- do not reintroduce a second live clock implementation without a tracked reason
- keep Python limited to verification and snapshot comparison
