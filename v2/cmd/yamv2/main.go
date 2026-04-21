package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/charmbracelet/bubbles/help"
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/mattn/go-runewidth"
)

type sceneConfig struct {
	ClockFontName string `json:"clock_font_name"`
	DayFormat     string `json:"day_format"`
	ClockFormat   string `json:"clock_format"`
	GifPath       string `json:"gif_path"`
	ThemeName     string `json:"theme_name"`
}

type configState struct {
	path string
	mod  time.Time
	cfg  sceneConfig
}

type tickMsg time.Time
type reloadMsg struct{}

type model struct {
	repoRoot      string
	cfgPath       string
	state         configState
	now           time.Time
	width         int
	height        int
	paused        bool
	clockOverride string
	dayOverride   string
	help          help.Model
	keys          keyMap
}

type keyMap struct {
	Quit  key.Binding
	Pause key.Binding
}

func (k keyMap) ShortHelp() []key.Binding {
	return []key.Binding{k.Quit, k.Pause}
}

func (k keyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{{k.Quit, k.Pause}}
}

func defaultConfig() sceneConfig {
	return sceneConfig{
		ClockFontName: "Fender",
		DayFormat:     "%A, %d %B",
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
		state.cfg.DayFormat = "%A, %d %B"
	}
	if state.cfg.ClockFormat == "" {
		state.cfg.ClockFormat = "%H:%M"
	}
	if state.cfg.ClockFontName == "" {
		state.cfg.ClockFontName = "Fender"
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
	clock := m.clockOverride
	if clock == "" {
		clock = m.now.Format(clockLayout)
	}
	if clock == "" {
		clock = time.Now().Format(clockLayout)
	}
	if m.clockOverride == "" && m.now.Second()%2 == 1 {
		clock = strings.ReplaceAll(clock, ":", " ")
	}
	day := m.dayOverride
	if day == "" {
		day = polishDayLabel(m.now)
	}
	if day == "" {
		day = polishDayLabel(time.Now())
	}

	footer := m.help.View(m.keys)
	return renderScene(width, height, clock, day, footer, m.state.cfg.ClockFontName, m.repoRoot)
}

func polishDayLabel(t time.Time) string {
	weekday := map[time.Weekday]string{
		time.Monday:    "poniedziałek",
		time.Tuesday:   "wtorek",
		time.Wednesday: "środa",
		time.Thursday:  "czwartek",
		time.Friday:    "piątek",
		time.Saturday:  "sobota",
		time.Sunday:    "niedziela",
	}[t.Weekday()]
	month := map[time.Month]string{
		time.January:   "stycznia",
		time.February:  "lutego",
		time.March:     "marca",
		time.April:     "kwietnia",
		time.May:       "maja",
		time.June:      "czerwca",
		time.July:      "lipca",
		time.August:    "sierpnia",
		time.September: "września",
		time.October:   "października",
		time.November:  "listopada",
		time.December:  "grudnia",
	}[t.Month()]
	return fmt.Sprintf("%s, %d %s", weekday, t.Day(), month)
}

func renderScene(width, height int, clock, day, footer, fontName, repoRoot string) string {
	if width < 24 {
		width = 24
	}
	if height < 16 {
		height = 16
	}

	rows := make([][]rune, height)
	for y := range rows {
		rows[y] = make([]rune, width)
		for x := range rows[y] {
			rows[y][x] = ' '
		}
	}

	placeBlock := func(x, y int, block string) {
		lines := strings.Split(block, "\n")
		for dy, line := range lines {
			if y+dy < 0 || y+dy >= height {
				continue
			}
			for dx, r := range line {
				if x+dx < 0 || x+dx >= width {
					continue
				}
				rows[y+dy][x+dx] = r
			}
		}
	}

	fontDir := figletFontDir(repoRoot)
	clockArt := renderFigletBlock(clock, fontDir, fontName)
	clockWidth := renderClockLineWidth(clock, fontDir, fontName)
	clockX := max(0, (width*3)/4-clockWidth/2)
	clockY := max(0, height/4)
	placeBlock(clockX, clockY, clockArt)

	dayY := clockY + 6
	dayX := max(0, clockX+(clockWidth-len(day))/2)
	placeBlock(dayX, dayY, day)

	footerY := max(0, height-2)
	footerX := max(0, (width-runewidth.StringWidth(footer))/2)
	placeBlock(footerX, footerY, footer)

	var b strings.Builder
	for _, row := range rows {
		b.WriteString(string(row))
		b.WriteByte('\n')
	}
	return strings.TrimRight(b.String(), "\n")
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
	var width int
	var height int
	var once bool
	var clockOverride string
	var dayOverride string
	flag.StringVar(&cfgPath, "config", "", "scene config json path")
	flag.IntVar(&width, "width", 40, "render width for one-shot mode")
	flag.IntVar(&height, "height", 20, "render height for one-shot mode")
	flag.BoolVar(&once, "once", false, "render a single frame and exit")
	flag.StringVar(&clockOverride, "clock", "", "override clock text for one-shot rendering")
	flag.StringVar(&dayOverride, "day", "", "override day text for one-shot rendering")
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
		repoRoot:      repoRoot,
		cfgPath:       cfgPath,
		state:         state,
		now:           time.Now(),
		width:         width,
		height:        height,
		clockOverride: clockOverride,
		dayOverride:   dayOverride,
		help:          help.New(),
		keys: keyMap{
			Quit:  key.NewBinding(key.WithKeys("q", "ctrl+c", "esc"), key.WithHelp("q", "quit")),
			Pause: key.NewBinding(key.WithKeys("space"), key.WithHelp("space", "pause")),
		},
	}

	if once {
		fmt.Print(m.View())
		return
	}

	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "yamv2: %v\n", err)
		os.Exit(1)
	}
}
