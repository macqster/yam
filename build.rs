use std::{path::PathBuf, process::Command};

fn git_output(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn watch_git_identity() {
    if let Some(head_path) = git_output(&["rev-parse", "--git-path", "HEAD"]) {
        println!("cargo:rerun-if-changed={head_path}");
    }

    if let Some(head_ref) = git_output(&["symbolic-ref", "-q", "HEAD"]) {
        if let Some(ref_path) = git_output(&["rev-parse", "--git-path", &head_ref]) {
            println!("cargo:rerun-if-changed={ref_path}");
        }
    }

    if let Some(git_dir) = git_output(&["rev-parse", "--git-common-dir"]) {
        println!(
            "cargo:rerun-if-changed={}",
            PathBuf::from(git_dir).join("packed-refs").display()
        );
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    watch_git_identity();
    let build_time = chrono::Utc::now().format("%y%m%d-%H%M").to_string();
    let git_hash =
        git_output(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "nogit".to_string());
    println!("cargo:rustc-env=YAM_BUILD_TIME={build_time}");
    println!("cargo:rustc-env=YAM_GIT_HASH={git_hash}");
}
