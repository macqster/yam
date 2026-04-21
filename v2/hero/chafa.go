package hero

import (
	"bytes"
	"fmt"
	"os/exec"
)

// ChafaRenderer shells out to the chafa CLI so the hero pipeline stays terminal-native.
type ChafaRenderer struct{}

// RenderFrame converts a GIF into a terminal art block using chafa.
func (r ChafaRenderer) RenderFrame(gifPath string, width int, height int) (string, error) {
	args := []string{
		"--format=symbols",
		"--symbols=block",
		"--size", fmt.Sprintf("%dx%d", width, height),
		gifPath,
	}
	cmd := exec.Command("chafa", args...)
	var out bytes.Buffer
	cmd.Stdout = &out
	cmd.Stderr = &out
	if err := cmd.Run(); err != nil {
		return "", fmt.Errorf("chafa render: %w: %s", err, out.String())
	}
	return out.String(), nil
}
