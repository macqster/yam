use chrono::Local;

pub fn current_time_string() -> String {
    Local::now().format("%H:%M").to_string()
}
