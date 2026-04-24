mod core;
mod render;
mod runtime;
mod scene;
mod theme;
mod systems;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==============================");
    println!("YAM RUNTIME IDENTITY");
    println!("SOURCE: {}", env!("CARGO_MANIFEST_DIR"));
    println!("VERSION: {}", env!("CARGO_PKG_VERSION"));
    println!("BUILD: {} {}", env!("YAM_BUILD_TIME"), env!("YAM_GIT_HASH"));
    println!("BIN PATH: {:?}", std::env::current_exe().unwrap());
    println!("==============================");
    if std::env::args().any(|a| a == "--version") {
        println!("yam-rust 0.3");
        return Ok(());
    }
    runtime::run()
}
