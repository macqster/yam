//! RUNTIME
//!
//! Rules:
//! - Event loop only
//! - Input handling
//! - Calls systems + render
//! - No layout logic

use std::{
    io,
    time::{Duration, Instant},
};

use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::systems::tick::tick;
use crate::ui::scene::render_scene;
use crate::ui::state::UiState;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut world = WorldState::new();
    let mut ui_state = UiState::new();
    let fonts = FontRegistry::new();
    let tick_rate = Duration::from_millis(250);

    loop {
        let start = Instant::now();
        tick(&mut world);
        ui_state.hero.tick();
        terminal.draw(|frame| {
            render_scene(frame, &world, &ui_state, &fonts);
        })?;

        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('d') => ui_state.toggle_debug_layout(),
                    KeyCode::Char('c') => ui_state.toggle_clock_mode(),
                    KeyCode::Char('f') => ui_state.toggle_follow_hero(),
                    KeyCode::Left => ui_state.camera.move_by(-2, 0),
                    KeyCode::Right => ui_state.camera.move_by(2, 0),
                    KeyCode::Up => ui_state.camera.move_by(0, -1),
                    KeyCode::Down => ui_state.camera.move_by(0, 1),
                    KeyCode::Char('h') => ui_state.move_hero_offset_left(),
                    KeyCode::Char('l') => ui_state.move_hero_offset_right(),
                    KeyCode::Char('k') => ui_state.move_hero_offset_up(),
                    KeyCode::Char('j') => ui_state.move_hero_offset_down(),
                    KeyCode::F(5) => ui_state.next_font(),
                    KeyCode::Char('}') => ui_state.next_font(),
                    KeyCode::Char('{') => ui_state.prev_font(),
                    KeyCode::Char(']') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                        ui_state.next_font()
                    }
                    KeyCode::Char('[') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                        ui_state.prev_font()
                    }
                    _ => {}
                }
            }
        }

        let elapsed = start.elapsed();
        if elapsed < tick_rate {
            std::thread::sleep(tick_rate - elapsed);
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
