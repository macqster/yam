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
use crate::scene::render_scene;
use crate::systems::tick::tick;
use crate::ui::state::UiState;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
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
    let mut ui_state = UiState::load_or_new();
    let fonts = FontRegistry::new();
    let world_tick = Duration::from_millis(250);
    let frame_time = Duration::from_secs_f64(1.0 / 120.0);
    let mut last_world_tick = Instant::now();
    let mut last_hero_tick = Instant::now();
    'run: loop {
        let frame_start = Instant::now();

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('q') => break 'run,
                    KeyCode::Char('d') => {
                        ui_state.toggle_debug_layout();
                    }
                    KeyCode::Char('f') => ui_state.toggle_follow_hero(),
                    KeyCode::Char(' ') => ui_state.hero.toggle_animation(),
                    KeyCode::Char('.') => ui_state.hero.step_animation(),
                    KeyCode::Left if ui_state.debug_layout => {
                        ui_state.move_camera_left();
                    }
                    KeyCode::Right if ui_state.debug_layout => {
                        ui_state.move_camera_right();
                    }
                    KeyCode::Up if ui_state.debug_layout => {
                        ui_state.move_camera_up();
                    }
                    KeyCode::Down if ui_state.debug_layout => {
                        ui_state.move_camera_down();
                    }
                    KeyCode::Char(c) => {
                        if ui_state.debug_layout {
                            let is_shift = modifiers.contains(KeyModifiers::SHIFT);
                            let base = c.to_ascii_lowercase();
                            match base {
                                'c' => ui_state.center_camera(),
                                'h' => {
                                    if is_shift || c.is_uppercase() {
                                        ui_state.adjust_clock_offset(-1, 0)?
                                    } else {
                                        ui_state.move_hero_offset_left()
                                    }
                                }
                                'j' => {
                                    if is_shift || c.is_uppercase() {
                                        ui_state.adjust_clock_offset(0, 1)?
                                    } else {
                                        ui_state.move_hero_offset_down()
                                    }
                                }
                                'k' => {
                                    if is_shift || c.is_uppercase() {
                                        ui_state.adjust_clock_offset(0, -1)?
                                    } else {
                                        ui_state.move_hero_offset_up()
                                    }
                                }
                                'l' => {
                                    if is_shift || c.is_uppercase() {
                                        ui_state.adjust_clock_offset(1, 0)?
                                    } else {
                                        ui_state.move_hero_offset_right()
                                    }
                                }
                                _ => {}
                            }
                            if c == 'd' {
                                ui_state.toggle_debug_layout();
                            } else if c == '}' || (c == '=' && is_shift) {
                                ui_state.next_font();
                            } else if c == '{' || (c == '-' && is_shift) {
                                ui_state.prev_font();
                            } else if c == '+' || c == '=' && is_shift {
                                ui_state.increase_hero_fps();
                            } else if c == '-' {
                                ui_state.decrease_hero_fps();
                            }
                        } else if c == 'c' {
                            ui_state.toggle_clock_mode();
                        }
                    }
                    KeyCode::F(5) if ui_state.debug_layout => ui_state.next_font(),
                    _ => {}
                }
            }
        }

        if frame_start.duration_since(last_world_tick) >= world_tick {
            tick(&mut world);
            last_world_tick = frame_start;
        }

        let hero_frame_time =
            Duration::from_secs_f64(1.0 / ui_state.offsets.hero_fps.max(0.5) as f64);
        if frame_start.duration_since(last_hero_tick) >= hero_frame_time {
            ui_state.hero.tick();
            last_hero_tick = frame_start;
        }

        terminal.autoresize().ok();
        terminal.draw(|frame| {
            render_scene(frame, &world, &ui_state, &fonts);
        })?;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }

        let frame_elapsed = frame_start.elapsed().as_secs_f64();
        ui_state.fps = if frame_elapsed > 0.0 {
            1.0 / frame_elapsed
        } else {
            0.0
        };
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
