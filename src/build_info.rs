pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = env!("YAM_BUILD_TIME");

pub fn build_hash() -> &'static str {
    option_env!("GIT_HASH").unwrap_or(env!("YAM_GIT_HASH"))
}
