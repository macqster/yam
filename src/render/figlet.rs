use figlet_rs::FIGlet;

pub fn render_figlet(font: &FIGlet, text: &str) -> Vec<String> {
    match font.convert(text) {
        Some(fig) => fig.to_string().lines().map(|s| s.to_string()).collect(),
        None => vec![text.to_string()],
    }
}
