package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

type sceneConfig struct {
	ClockFontPath string `json:"clock_font_path"`
	DayFormat     string `json:"day_format"`
	ClockFormat   string `json:"clock_format"`
	GifPath       string `json:"gif_path"`
	ThemeName     string `json:"theme_name"`
}

type configState struct {
	path    string
	mod     time.Time
	cfg     sceneConfig
}

type tickMsg time.Time
type reloadMsg struct{}

type model struct {
	repoRoot string
	cfgPath  string
	state    configState
	now      time.Time
	width    int
	height   int
	paused   bool
}

func defaultConfig() sceneConfig {
	return sceneConfig{
		ClockFontPath: "v2/assets/fonts/Gothic.flf",
		DayFormat:     "%A",
		ClockFormat:   "%H:%M",
		GifPath:       "visualizer/assets/source.gif",
		ThemeName:     "btas_dark_deco",
	}
}

func resolveRelative(root, value string) string {
	if filepath.IsAbs(value) {
		return value
	}
	return filepath.Join(root, value)
}

func loadConfig(repoRoot, path string) (configState, error) {
	state := configState{path: path, cfg: defaultConfig()}
	info, err := os.Stat(path)
	if err != nil {
		if os.IsNotExist(err) {
			return state, nil
		}
		return state, err
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return state, err
	}
	if err := json.Unmarshal(data, &state.cfg); err != nil {
		return state, err
	}
	if state.cfg.DayFormat == "" {
		state.cfg.DayFormat = "%A"
	}
	if state.cfg.ClockFormat == "" {
		state.cfg.ClockFormat = "%H:%M"
	}
	if state.cfg.ClockFontPath == "" {
		state.cfg.ClockFontPath = "v2/assets/fonts/Gothic.flf"
	}
	state.mod = info.ModTime()
	return state, nil
}

func maybeReload(repoRoot, path string, prev configState) (configState, tea.Cmd) {
	info, err := os.Stat(path)
	if err != nil {
		return prev, nil
	}
	if info.ModTime().Equal(prev.mod) {
		return prev, nil
	}
	next, err := loadConfig(repoRoot, path)
	if err != nil {
		stale := prev
		stale.cfg = sceneConfig{ThemeName: "config-error"}
		return stale, func() tea.Msg { return reloadMsg{} }
	}
	return next, func() tea.Msg { return reloadMsg{} }
}

func tickCmd() tea.Cmd {
	return tea.Tick(time.Second, func(t time.Time) tea.Msg {
		return tickMsg(t)
	})
}

func (m model) Init() tea.Cmd {
	return tea.Batch(tickCmd(), func() tea.Msg { return reloadMsg{} })
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "q", "ctrl+c", "esc":
			return m, tea.Quit
		case " ":
			m.paused = !m.paused
			return m, nil
		}
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		return m, nil
	case tickMsg:
		m.now = time.Time(msg)
		if m.paused {
			return m, tickCmd()
		}
		reloaded, reloadCmd := maybeReload(m.repoRoot, m.cfgPath, m.state)
		m.state = reloaded
		return m, tea.Batch(tickCmd(), reloadCmd)
	case reloadMsg:
		reloaded, reloadCmd := maybeReload(m.repoRoot, m.cfgPath, m.state)
		m.state = reloaded
		return m, reloadCmd
	case tea.QuitMsg:
		return m, tea.Quit
	}
	return m, nil
}

func (m model) View() string {
	width := m.width
	height := m.height
	if width == 0 {
		width = 80
	}
	if height == 0 {
		height = 24
	}

	clockLayout := translateClockFormat(m.state.cfg.ClockFormat)
	dayLayout := translateClockFormat(m.state.cfg.DayFormat)
	clock := m.now.Format(clockLayout)
	if clock == "" {
		clock = time.Now().Format(clockLayout)
	}
	day := m.now.Format(dayLayout)
	if day == "" {
		day = time.Now().Format(dayLayout)
	}

	clockArt := renderBreamDecoClock(clock)
	content := lipgloss.JoinVertical(lipgloss.Center, clockArt, day)
	panel := lipgloss.NewStyle().
		Width(width).
		Height(height).
		Align(lipgloss.Center).
		AlignVertical(lipgloss.Center).
		Render(content)
	return panel
}

func translateClockFormat(layout string) string {
	replacements := []struct {
		from string
		to   string
	}{
		{"%H", "15"},
		{"%M", "04"},
		{"%S", "05"},
		{"%d", "02"},
		{"%m", "01"},
		{"%Y", "2006"},
		{"%a", "Mon"},
		{"%A", "Monday"},
	}
	out := layout
	for _, repl := range replacements {
		out = strings.ReplaceAll(out, repl.from, repl.to)
	}
	return out
}

func main() {
	var cfgPath string
	flag.StringVar(&cfgPath, "config", "", "scene config json path")
	flag.Parse()

	repoRoot := os.Getenv("YAM_REPO")
	if repoRoot == "" {
		repoRoot = filepath.Clean(filepath.Join(filepath.Dir(os.Args[0]), "..", ".."))
	}
	if cfgPath == "" {
		cfgPath = filepath.Join(repoRoot, "v2", "scene_config.json")
	}

	state, err := loadConfig(repoRoot, cfgPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "yamv2: load config: %v\n", err)
		os.Exit(1)
	}

	m := model{
		repoRoot: repoRoot,
		cfgPath:  cfgPath,
		state:    state,
		now:      time.Now(),
	}

	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "yamv2: %v\n", err)
		os.Exit(1)
	}
}
