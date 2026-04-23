use std::process::Command;
use std::sync::OnceLock;

const HERO_GIF_PATH: &str = "/Users/maciejkuster/Desktop/hero_gif_1.gif";

static HERO_FRAME: OnceLock<Vec<String>> = OnceLock::new();

pub fn render_frame(path: &str, width: u16, height: u16) -> Vec<String> {
    let size_arg = format!("{}x{}", width, height);
    let output = Command::new("chafa")
        .arg(path)
        .arg("--size")
        .arg(size_arg)
        .arg("--format=symbols")
        .arg("--symbols=braille")
        .arg("--animate=off")
        .output()
        .unwrap_or_else(|err| panic!("failed to run chafa: {err}"));

    if !output.status.success() {
        return vec![format!("chafa exited with status {}", output.status)];
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    sanitize_lines(&stdout)
}

pub fn hero_frame(width: u16, height: u16) -> Vec<String> {
    HERO_FRAME
        .get_or_init(|| render_frame(HERO_GIF_PATH, width, height))
        .clone()
}

fn sanitize_lines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for raw_line in text.lines() {
        let mut line = String::with_capacity(raw_line.len());
        let mut chars = raw_line.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\u{1b}' {
                if matches!(chars.peek(), Some('[')) {
                    let _ = chars.next();
                    for next in chars.by_ref() {
                        if ('@'..='~').contains(&next) {
                            break;
                        }
                    }
                }
                continue;
            }

            if ch.is_control() && ch != '\t' {
                continue;
            }

            line.push(ch);
        }

        if !line.is_empty() {
            lines.push(line);
        }
    }
    lines
}
