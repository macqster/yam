mod core;
mod systems;
mod ui;
mod render;
mod runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a == "--version") {
        println!("yam-rust 0.3");
        return Ok(());
    }
    runtime::run()
}
