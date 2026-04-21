package main

import (
	"path/filepath"
	"strings"

	figlet "github.com/lsferreira42/figlet-go/figlet"
)

func renderFigletBlock(text, fontDir, fontName string) string {
	out, err := figlet.Render(text,
		figlet.WithFontDir(fontDir),
		figlet.WithFont(fontName),
		figlet.WithSmushing(),
		figlet.WithWidth(0),
	)
	if err == nil {
		return out
	}
	fallback, fallbackErr := figlet.Render(text)
	if fallbackErr == nil {
		return fallback
	}
	return text
}

func renderClockLineWidth(text, fontDir, fontName string) int {
	block := renderFigletBlock(text, fontDir, fontName)
	width := 0
	for _, line := range strings.Split(block, "\n") {
		if len(line) > width {
			width = len(line)
		}
	}
	return width
}

func figletFontDir(repoRoot string) string {
	return filepath.Join(repoRoot, "v2", "render", "fonts")
}
