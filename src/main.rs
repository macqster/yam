mod build_info;
mod core;
mod render;
mod runtime;
mod scene;
mod systems;
mod theme;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fn run(cmd: &str, args: &[&str]) {
        std::process::Command::new(cmd)
            .args(args)
            .status()
            .expect("failed to run command");
    }

    let args: Vec<String> = std::env::args().collect();
    println!("==============================");
    println!("YAM RUNTIME IDENTITY");
    println!("SOURCE: {}", env!("CARGO_MANIFEST_DIR"));
    println!(
        "BUILD: yam {}, build {} ({})",
        build_info::VERSION,
        build_info::BUILD_TIME,
        build_info::build_hash()
    );
    println!("BIN PATH: {:?}", std::env::current_exe().unwrap());
    println!("==============================");
    if args.iter().any(|a| a == "--version") {
        println!("yam-rust {}", build_info::VERSION);
        return Ok(());
    }
    if args.iter().any(|a| a == "--check-updates") {
        run("cargo", &["outdated"]);
        return Ok(());
    }
    if args.iter().any(|a| a == "--update") {
        run("bash", &["scripts/update.sh"]);
        return Ok(());
    }
    runtime::run()
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
    fn scene_config_matches_the_current_runtime_defaults() {
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
