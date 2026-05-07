use figlet_rs::FIGlet;

pub fn render_figlet(font: &FIGlet, text: &str) -> Vec<String> {
    match font.convert(text) {
        Some(fig) => {
            trim_trailing_empty_lines(fig.to_string().lines().map(|s| s.to_string()).collect())
        }
        None => vec![text.to_string()],
    }
}

fn trim_trailing_empty_lines(mut lines: Vec<String>) -> Vec<String> {
    while lines.last().is_some_and(|line| line.trim().is_empty()) {
        lines.pop();
    }
    lines
}
