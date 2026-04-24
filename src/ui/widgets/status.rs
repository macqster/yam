pub fn build_status_label() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let build_time = env!("YAM_BUILD_TIME");
    format!("yam {version}, build {build_time}")
}
