use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let build_time = Command::new("date")
        .arg("-u")
        .arg("+%y%m%d–%H%M")
        .output()
        .unwrap_or_else(|err| panic!("failed to run date: {err}"));
    let build_time = String::from_utf8_lossy(&build_time.stdout).trim().to_string();
    println!("cargo:rustc-env=YAM_BUILD_TIME={build_time}");
    println!("cargo:rustc-env=YAM_GIT_HASH=TESTHASH");
}
