fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=YAM_BUILD_TIME=FORCED_BUILD_TEST");
    println!("cargo:rustc-env=YAM_GIT_HASH=TESTHASH");
}
