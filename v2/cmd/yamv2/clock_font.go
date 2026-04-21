package main

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

var breamDecoFont = map[rune][]string{}

func loadBreamDecoFont(repoRoot string) error {
	path := filepath.Join(repoRoot, "v2", "render", "fonts", "go_deco.txt")
	data, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("read clock font: %w", err)
	}
	breamDecoFont = parseBreamDecoFont(string(data))
	return nil
}

func parseBreamDecoFont(source string) map[rune][]string {
	font := make(map[rune][]string)
	rowPattern := regexp.MustCompile(`"([^"]*)"`)
	blocks := strings.Split(source, "\n\n")
	for _, block := range blocks {
		lines := strings.Split(strings.TrimSpace(block), "\n")
		if len(lines) == 0 || strings.TrimSpace(lines[0]) == "" {
			continue
		}
		header := lines[0]
		if len(header) < 3 {
			continue
		}
		key := rune(header[1])
		glyph := make([]string, 0, len(lines)-1)
		for _, line := range lines[1:] {
			match := rowPattern.FindStringSubmatch(line)
			if match == nil {
				continue
			}
			glyph = append(glyph, match[1])
		}
		if len(glyph) == 7 {
			font[key] = glyph
		}
	}
	return font
}

func renderBreamDecoClock(text string) string {
	rows := make([]string, 7)
	for _, ch := range text {
		glyph, ok := breamDecoFont[ch]
		if !ok {
			continue
		}
		for i := 0; i < 7; i++ {
			rows[i] += glyph[i] + " "
		}
	}
	return strings.Join(rows, "\n")
}
