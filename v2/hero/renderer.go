package hero

// Renderer converts a GIF asset into terminal-ready text frames.
// The live scene is expected to keep this layer separate from the clock and engine.
type Renderer interface {
	RenderFrame(gifPath string, width int, height int) (string, error)
}
