use crate::build_info;

pub fn build_status_label() -> String {
    format!("yam {} ({})", build_info::VERSION, build_info::build_hash())
}
