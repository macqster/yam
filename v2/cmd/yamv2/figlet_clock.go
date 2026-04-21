package main

import (
	"strings"

	figlet "github.com/lsferreira42/figlet-go/figlet"
)

func renderFigletBlock(text, fontName string) string {
	out, err := figlet.RenderWithFont(text, fontName)
	if err == nil {
		return out
	}
	fallback, fallbackErr := figlet.Render(text)
	if fallbackErr == nil {
		return fallback
	}
	return text
}

func renderClockLineWidth(text, fontName string) int {
	block := renderFigletBlock(text, fontName)
	width := 0
	for _, line := range strings.Split(block, "\n") {
		if len(line) > width {
			width = len(line)
		}
	}
	return width
}
