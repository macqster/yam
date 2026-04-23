mod core;
mod render;
mod runtime;
mod systems;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a == "--version") {
        println!("yam-rust 0.3");
        return Ok(());
    }
    runtime::run()
}
