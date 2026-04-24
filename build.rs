use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let build_time = Command::new("date")
        .arg("-u")
        .arg("+%Y-%m-%d_%H:%M:%S")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=YAM_BUILD_TIME={build_time}");

    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=YAM_GIT_HASH={git_hash}");
}
