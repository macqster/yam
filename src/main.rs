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
