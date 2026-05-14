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

use serde_json::json;

use crate::core::world::{WorldKind, WorldState};
use crate::diagnostics::append_event;
use crate::render::compositor::Grid;
use crate::render::fonts::FontRegistry;
use crate::scene::{render_scene_with_scene_and_grid, Scene};
use crate::systems::tick::tick;
use crate::ui::scene::build_ui_layers;
use crate::ui::state::UiState;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tachyonfx::{fx, CellFilter, Effect, Interpolation};

fn build_loading_effect(phase: crate::ui::state::BootLoadingPhase) -> Effect {
    let mut effect = match phase {
        crate::ui::state::BootLoadingPhase::Coalesce => {
            fx::coalesce((1000, Interpolation::BounceInOut))
        }
        crate::ui::state::BootLoadingPhase::Dissolve => fx::dissolve(1000),
        crate::ui::state::BootLoadingPhase::Bar
        | crate::ui::state::BootLoadingPhase::AwaitStart
        | crate::ui::state::BootLoadingPhase::Hold => {
            unreachable!("only effect phases should build tachyonfx effects")
        }
    };
    effect.filter(CellFilter::Text);
    effect
}

pub fn run(
    initial_world_kind: WorldKind,
    clean_launch: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let runtime_start = Instant::now();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ui_state = UiState::load_or_new();
    if clean_launch {
        ui_state.reset_for_clean_launch(initial_world_kind);
    }
    if ui_state.active_world_kind() != initial_world_kind {
        ui_state.force_initial_world_kind(initial_world_kind);
    }
    ui_state.refresh_weather_if_due();
    ui_state.start_loading_boot();
    let mut world = WorldState::for_kind(ui_state.active_world_kind());
    let boot_world = WorldState::for_boot();
    let fonts = FontRegistry::new();
    let scene = Scene::new(build_ui_layers());
    let mut final_grid = Grid::new(0, 0);
    let world_tick = Duration::from_millis(250);
    let frame_time = Duration::from_secs_f64(1.0 / 120.0);
    let pointer_blink = Duration::from_millis(420);
    let mut last_terminal_size = terminal.size()?;
    let mut last_world_tick = Instant::now();
    let mut last_hero_tick = Instant::now();
    let mut last_pointer_blink = Instant::now();
    let mut loading_effect_phase = None;
    let mut loading_effect: Option<Effect> = None;
    let mut loading_effect_last_tick = Instant::now();
    let mut last_boot_phase = ui_state.loading.boot_phase();
    let mut logged_world_ready = false;

    append_event(
        "boot_start",
        &[
            ("initial_world", json!(initial_world_kind.title())),
            ("clean_launch", json!(clean_launch)),
            ("version", json!(crate::build_info::VERSION)),
            ("build", json!(crate::build_info::build_hash())),
        ],
    );
    if let Some(phase) = last_boot_phase {
        append_event("boot_phase", &[("phase", json!(format!("{phase:?}")))]);
    }

    'run: loop {
        let frame_start = Instant::now();
        ui_state.update_loading();
        let boot_phase = ui_state.loading.boot_phase();
        if boot_phase != last_boot_phase {
            if let Some(phase) = boot_phase {
                append_event("boot_phase", &[("phase", json!(format!("{phase:?}")))]);
            } else if last_boot_phase.is_some() {
                append_event(
                    "world_ready",
                    &[
                        ("boot_ms", json!(runtime_start.elapsed().as_millis() as u64)),
                        ("world", json!(ui_state.active_world_kind().title())),
                    ],
                );
                logged_world_ready = true;
            }
            last_boot_phase = boot_phase;
        }
        ui_state.refresh_weather_if_due();

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                if ui_state.loading.active {
                    match code {
                        KeyCode::Char('q') => break 'run,
                        KeyCode::Char('d') => ui_state.toggle_dev_mode(),
                        KeyCode::Char(' ') if ui_state.loading.awaiting_start_confirmation() => {
                            ui_state.acknowledge_loading_start();
                        }
                        _ => {}
                    }
                    continue;
                }

                match code {
                    KeyCode::Char('d')
                        if ui_state.quit_confirm_active()
                            && ui_state.confirm_quit_without_saving() =>
                    {
                        break 'run;
                    }
                    KeyCode::Char('s')
                        if ui_state.quit_confirm_active() && ui_state.confirm_save_and_quit() =>
                    {
                        break 'run;
                    }
                    KeyCode::Char('q') if ui_state.begin_quit() => {
                        break 'run;
                    }
                    KeyCode::Char('d') => ui_state.toggle_dev_mode(),
                    KeyCode::Esc if ui_state.handle_global_escape() => {}
                    KeyCode::Char('?') if ui_state.global_help_active() => {
                        ui_state.toggle_help_globally()
                    }
                    KeyCode::Char('f') => ui_state.toggle_follow_hero(),
                    KeyCode::Char('C') if ui_state.dev_free_roam() => {
                        ui_state.store_camera_home();
                    }
                    KeyCode::Char('p') if ui_state.dev_free_roam() => {
                        ui_state.toggle_pointer_probe();
                    }
                    KeyCode::Char('P') if ui_state.palette_toggle_allowed() => {
                        ui_state.toggle_palette();
                    }
                    KeyCode::Char('W') if ui_state.weather_popup_toggle_allowed() => {
                        ui_state.toggle_weather_popup();
                    }
                    KeyCode::Char('m') if ui_state.move_mode_toggle_allowed() => {
                        ui_state.toggle_move_mode()
                    }
                    KeyCode::Char('w') if ui_state.dev_free_roam() => {
                        ui_state.cycle_world_kind();
                        world = WorldState::for_kind(ui_state.active_world_kind());
                    }
                    KeyCode::Char('v') if ui_state.dev_free_roam() => {
                        ui_state.toggle_vines_visible();
                    }
                    KeyCode::Char('s') if ui_state.settings_toggle_allowed() => {
                        ui_state.toggle_settings()
                    }
                    KeyCode::Enter if ui_state.settings_navigation_active() => {
                        let size = terminal.size()?;
                        ui_state.activate_selected_setting_with_viewport(
                            size.width,
                            size.height.saturating_sub(1),
                        )?;
                    }
                    KeyCode::Char(' ') => ui_state.hero.toggle_animation(),
                    KeyCode::Char('.') => ui_state.hero.step_animation(),
                    KeyCode::Left if ui_state.settings_edit_active() => {
                        ui_state.toggle_settings_edit_field();
                    }
                    KeyCode::Left if ui_state.move_mode_active() => {
                        ui_state.move_selected_target_left()?;
                    }
                    KeyCode::Left if ui_state.dev_free_roam() => {
                        if ui_state.pointer_probe_active() {
                            ui_state.move_pointer_left();
                        } else {
                            ui_state.move_camera_left();
                        }
                    }
                    KeyCode::Right if ui_state.settings_edit_active() => {
                        ui_state.toggle_settings_edit_field();
                    }
                    KeyCode::Right if ui_state.move_mode_active() => {
                        ui_state.move_selected_target_right()?;
                    }
                    KeyCode::Right if ui_state.dev_free_roam() => {
                        if ui_state.pointer_probe_active() {
                            ui_state.move_pointer_right();
                        } else {
                            ui_state.move_camera_right();
                        }
                    }
                    KeyCode::Up if ui_state.settings_navigation_active() => {
                        ui_state.select_prev_settings_row();
                    }
                    KeyCode::Up if ui_state.move_mode_active() => {
                        ui_state.move_selected_target_up()?;
                    }
                    KeyCode::Down if ui_state.settings_navigation_active() => {
                        ui_state.select_next_settings_row();
                    }
                    KeyCode::Down if ui_state.move_mode_active() => {
                        ui_state.move_selected_target_down()?;
                    }
                    KeyCode::Up if ui_state.dev_free_roam() => {
                        if ui_state.pointer_probe_active() {
                            ui_state.move_pointer_up();
                        } else {
                            ui_state.move_camera_up();
                        }
                    }
                    KeyCode::Down if ui_state.dev_free_roam() => {
                        if ui_state.pointer_probe_active() {
                            ui_state.move_pointer_down();
                        } else {
                            ui_state.move_camera_down();
                        }
                    }
                    KeyCode::Tab if ui_state.settings_tab_switch_allowed() => {
                        ui_state.next_settings_tab();
                    }
                    KeyCode::Tab if ui_state.move_mode_active() => {
                        ui_state.next_move_target();
                    }
                    KeyCode::Tab if ui_state.debug_panel_tab_switch_allowed() => {
                        ui_state.next_debug_panel_tab();
                    }
                    KeyCode::Char(c) => {
                        if ui_state.settings_edit_active() {
                            ui_state.settings_edit_insert_char(c);
                        } else if ui_state.dev_free_roam() {
                            let is_shift = modifiers.contains(KeyModifiers::SHIFT);
                            let base = c.to_ascii_lowercase();
                            if base == 'c' {
                                ui_state.recall_camera_home();
                            }
                            if c == 'd' {
                                ui_state.toggle_dev_mode();
                            } else if c == '}' || (c == '=' && is_shift) {
                                ui_state.next_font();
                            } else if c == '{' || (c == '-' && is_shift) {
                                ui_state.prev_font();
                            } else if c == '+' || c == '=' && is_shift {
                                ui_state.increase_hero_fps();
                            } else if c == '-' {
                                ui_state.decrease_hero_fps();
                            }
                        }
                    }
                    KeyCode::BackTab if ui_state.settings_tab_switch_allowed() => {
                        ui_state.prev_settings_tab();
                    }
                    KeyCode::BackTab if ui_state.move_mode_active() => {
                        ui_state.prev_move_target();
                    }
                    KeyCode::BackTab if ui_state.debug_panel_tab_switch_allowed() => {
                        ui_state.prev_debug_panel_tab();
                    }
                    KeyCode::Backspace if ui_state.settings_edit_mode_active() => {
                        ui_state.settings_edit_backspace();
                    }
                    KeyCode::F(5) if ui_state.font_cycle_allowed() => ui_state.next_font(),
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

        if ui_state.should_blink_pointer_probe() {
            if frame_start.duration_since(last_pointer_blink) >= pointer_blink {
                ui_state.pointer_blink_on = !ui_state.pointer_blink_on;
                last_pointer_blink = frame_start;
            }
        } else {
            ui_state.pointer_blink_on = true;
            last_pointer_blink = frame_start;
        }

        terminal.autoresize().ok();
        let size = terminal.size()?;
        if size != last_terminal_size {
            if ui_state.camera.follow_hero {
                ui_state.sync_camera_to_viewport_center(size.width as i32, size.height as i32);
            } else {
                ui_state.preserve_camera_center_on_resize(
                    last_terminal_size.width as i32,
                    last_terminal_size.height as i32,
                    size.width as i32,
                    size.height as i32,
                );
            }
            last_terminal_size = size;
        }
        if ui_state.camera.follow_hero {
            ui_state.sync_camera_to_viewport_center(size.width as i32, size.height as i32);
        } else {
            ui_state.clamp_camera(size.width as i32, size.height as i32);
        }
        terminal.draw(|frame| {
            let render_world = if ui_state.loading.active {
                &boot_world
            } else {
                &world
            };
            render_scene_with_scene_and_grid(
                &scene,
                frame,
                render_world,
                &ui_state,
                &fonts,
                &mut final_grid,
            );
            if let Some(phase) = ui_state.loading.effect_phase() {
                if loading_effect_phase != Some(phase) {
                    loading_effect_phase = Some(phase);
                    loading_effect = Some(build_loading_effect(phase));
                    loading_effect_last_tick = frame_start;
                }
                if let Some(effect) = loading_effect.as_mut() {
                    let elapsed = frame_start.saturating_duration_since(loading_effect_last_tick);
                    let area = frame.area();
                    let _ = effect.process(elapsed, frame.buffer_mut(), area);
                    loading_effect_last_tick = frame_start;
                }
            } else {
                loading_effect_phase = None;
                loading_effect = None;
                loading_effect_last_tick = frame_start;
            }
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

    if !logged_world_ready {
        append_event(
            "runtime_exit",
            &[
                ("boot_completed", json!(!ui_state.loading.active)),
                (
                    "runtime_ms",
                    json!(runtime_start.elapsed().as_millis() as u64),
                ),
            ],
        );
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
