use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let build_time = chrono::Utc::now().format("%y%m%d-%H%M").to_string();
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|hash| hash.trim().to_string())
        .filter(|hash| !hash.is_empty())
        .unwrap_or_else(|| "nogit".to_string());
    println!("cargo:rustc-env=YAM_BUILD_TIME={build_time}");
    println!("cargo:rustc-env=YAM_GIT_HASH={git_hash}");
}
