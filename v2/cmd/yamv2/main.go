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
)

type sceneConfig struct {
	GifPath     string `json:"gif_path"`
	ClockFormat string `json:"clock_format"`
	ThemeName   string `json:"theme_name"`
}

type configState struct {
	path string
	mod  time.Time
	cfg  sceneConfig
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
	err      error
	paused   bool
}

func defaultConfig() sceneConfig {
	return sceneConfig{
		GifPath:     "visualizer/assets/source.gif",
		ClockFormat: "%H:%M",
		ThemeName:   "btas_dark_deco",
	}
}

func loadConfig(path string) (configState, error) {
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
	state.mod = info.ModTime()
	return state, nil
}

func maybeReload(path string, prev configState) (configState, tea.Cmd) {
	info, err := os.Stat(path)
	if err != nil {
		return prev, nil
	}
	if info.ModTime().Equal(prev.mod) {
		return prev, nil
	}
	next, err := loadConfig(path)
	if err != nil {
		stale := prev
		stale.cfg = sceneConfig{ThemeName: "config-error"}
		return stale, func() tea.Msg { return reloadMsg{} }
	}
	return next, func() tea.Msg { return reloadMsg{} }
}

func initialClock(layout string) string {
	return time.Now().Format(layout)
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
		reloaded, reloadCmd := maybeReload(m.cfgPath, m.state)
		m.state = reloaded
		return m, tea.Batch(tickCmd(), reloadCmd)
	case reloadMsg:
		reloaded, reloadCmd := maybeReload(m.cfgPath, m.state)
		m.state = reloaded
		return m, reloadCmd
	case tea.QuitMsg:
		return m, tea.Quit
	}
	return m, nil
}

func (m model) View() string {
	if m.width == 0 {
		return "loading..."
	}
	clock := m.now.Format(m.state.cfg.ClockFormat)
	if clock == "" {
		clock = initialClock(m.state.cfg.ClockFormat)
	}
	return renderScene(m.width, m.height, m.state.cfg.ThemeName, m.state.cfg.GifPath, clock, m.paused)
}

func ternary[T any](cond bool, yes, no T) T {
	if cond {
		return yes
	}
	return no
}

func renderScene(width, height int, theme, gifPath, clock string, paused bool) string {
	if width < 24 {
		width = 24
	}
	if height < 8 {
		height = 8
	}
	rows := make([][]rune, height)
	for y := range rows {
		rows[y] = make([]rune, width)
		for x := range rows[y] {
			rows[y][x] = ' '
		}
	}

	for x := 0; x < width; x++ {
		rows[0][x] = '─'
		rows[height-1][x] = '─'
	}
	for y := 0; y < height; y++ {
		rows[y][0] = '│'
		rows[y][width-1] = '│'
	}
	rows[0][0], rows[0][width-1], rows[height-1][0], rows[height-1][width-1] = '┌', '┐', '└', '┘'

	place := func(x, y int, text string) {
		if y < 0 || y >= height {
			return
		}
		for i, r := range text {
			if x+i < 0 || x+i >= width {
				continue
			}
			rows[y][x+i] = r
		}
	}

	title := "YAM v2 Bubble Tea runtime"
	place(2, 1, title)
	place(2, 3, "clock: "+clock)
	place(2, 4, "theme: "+theme)
	place(2, 5, "gif:   "+gifPath)
	place(2, 6, "mode:  "+ternary(paused, "paused", "live"))
	place(2, height-2, "press q to quit, space to pause")
	place(width-len(clock)-4, 1, clock)

	seed := len(theme) + len(gifPath) + len(clock)
	glyphs := []rune("·*:+=")
	for i := 0; i < width/3 && 2+i < width-2; i++ {
		y := 8 + (seed+i)%max(1, height-10)
		if y >= height-2 {
			y = height - 3
		}
		x := 2 + (i*3+seed)%max(1, width-6)
		rows[y][x] = glyphs[i%len(glyphs)]
	}
	if paused {
		place(width-12, height-2, "[paused]")
	}

	var b strings.Builder
	for _, row := range rows {
		b.WriteString(string(row))
		b.WriteByte('\n')
	}
	return strings.TrimRight(b.String(), "\n")
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
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
	}
	out := layout
	for _, repl := range replacements {
		out = strings.ReplaceAll(out, repl.from, repl.to)
	}
	if out == layout {
		return layout
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

	state, err := loadConfig(cfgPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "yamv2: load config: %v\n", err)
		os.Exit(1)
	}

	clockLayout := translateClockFormat(state.cfg.ClockFormat)

	m := model{
		repoRoot: repoRoot,
		cfgPath:   cfgPath,
		state:     state,
		now:       time.Now(),
	}
	m.state.cfg.ClockFormat = clockLayout

	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "yamv2: %v\n", err)
		os.Exit(1)
	}
}
