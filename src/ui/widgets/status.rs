use crate::build_info;

pub fn build_status_label() -> String {
    format!(
        "yam {}, build {} ({})",
        build_info::VERSION,
        build_info::BUILD_TIME,
        build_info::build_hash()
    )
}
