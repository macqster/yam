mod build_info;
mod core;
mod diagnostics;
mod render;
mod runtime;
mod scene;
mod systems;
mod theme;
mod ui;
mod weather;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fn run(cmd: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let status = std::process::Command::new(cmd).args(args).status()?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("{cmd} {} exited with status {status}", args.join(" ")).into())
        }
    }

    fn print_runtime_identity() -> Result<(), Box<dyn std::error::Error>> {
        println!("==============================");
        println!("YAM RUNTIME IDENTITY");
        println!("SOURCE: {}", env!("CARGO_MANIFEST_DIR"));
        println!(
            "BUILD: yam {}, build {} ({})",
            build_info::VERSION,
            build_info::BUILD_TIME,
            build_info::build_hash()
        );
        println!("BIN PATH: {:?}", std::env::current_exe()?);
        println!("==============================");
        Ok(())
    }

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version") {
        println!("yam-rust {}", build_info::VERSION);
        return Ok(());
    }
    if args.iter().any(|a| a == "--identity") {
        print_runtime_identity()?;
        return Ok(());
    }
    if args.iter().any(|a| a == "--check-updates") {
        return run("cargo", &["outdated"]);
    }
    if args.iter().any(|a| a == "--update") {
        return run("bash", &["scripts/update.sh"]);
    }
    if let Some(flag_pos) = args.iter().position(|a| a == "--compile-hero") {
        let mut options = render::hero_compiler::CompileOptions::default();
        if let Some(source) = args.get(flag_pos + 1) {
            if !source.starts_with("--") {
                options.source_path = std::path::PathBuf::from(source);
            }
        }
        return render::hero_compiler::compile(&options)
            .map(|_| ())
            .map_err(|err| err.into());
    }
    let initial_world_kind = if args.iter().any(|a| a == "--sandbox") {
        crate::core::world::WorldKind::Sandbox
    } else {
        crate::core::world::WorldKind::MainScene
    };
    // Saved dev positions are preserved by default. They are reseeded only on
    // an explicit `--hard-reset`, or automatically when the saved file was
    // written by a different version - see `saved_state_predates_this_version`.
    let hard_reset = args.iter().any(|a| a == "--hard-reset");
    runtime::run(initial_world_kind, hard_reset)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct SceneConfig {
        clock_font_name: String,
        day_format: String,
        gif_path: String,
        hero_anchor: String,
        hero_width: i32,
        hero_height: i32,
        hero_offset_x: i32,
        hero_offset_y: i32,
        clock_format: String,
        theme_name: String,
    }

    #[test]
    fn scene_config_matches_the_current_tooling_defaults() {
        let config: SceneConfig = serde_json::from_str(include_str!("../scene_config.json"))
            .expect("scene_config.json should be valid JSON");

        assert_eq!(config.clock_font_name, "Fender");
        assert_eq!(config.day_format, "%A, %d %B");
        assert_eq!(config.clock_format, "%H:%M");
        assert_eq!(config.gif_path, "hero/assets/hero_go.gif");
        assert_eq!(config.hero_anchor, "left");
        assert_eq!(config.hero_width, 10);
        assert_eq!(config.hero_height, 6);
        assert_eq!(config.hero_offset_x, 0);
        assert_eq!(config.hero_offset_y, 0);
        assert_eq!(config.theme_name, "btas_dark_deco");
    }
}
