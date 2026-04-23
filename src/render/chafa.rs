use std::{
    io::{BufReader, Read},
    process::{Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
};

use ansi_to_tui::IntoText;
use ratatui::text::Line;

const HERO_GIF_PATH: &str = "/Users/maciejkuster/Desktop/hero_gif_1.gif";

pub fn render_frame(path: &str, width: u16, height: u16) -> Vec<Line<'static>> {
    let size_arg = format!("{}x{}", width, height);
    let output = Command::new("chafa")
        .arg(path)
        .arg("--size")
        .arg(size_arg)
        .arg("--format=symbols")
        .arg("--symbols=braille")
        .arg("--colors=full")
        .arg("--color-space=din99d")
        .arg("--color-extractor=median")
        .arg("--dither=diffusion")
        .arg("--animate=off")
        .arg("--bg=#100100")
        .output()
        .unwrap_or_else(|err| panic!("failed to run chafa: {err}"));

    if !output.status.success() {
        return vec![format!("chafa exited with status {}", output.status).into()];
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .into_owned()
        .into_text()
        .expect("failed to convert ANSI text")
        .lines
}

pub fn spawn_chafa_stream(path: &str, width: u16, height: u16) -> Receiver<Vec<Line<'static>>> {
    let (tx, rx) = mpsc::channel();
    let path = path.to_string();

    thread::spawn(move || {
        let size_arg = format!("{}x{}", width, height);
        let mut child = Command::new("chafa")
            .arg(&path)
            .args([
                "--size",
                &size_arg,
                "--format=symbols",
                "--symbols=braille",
                "--colors=full",
                "--color-space=din99d",
                "--color-extractor=median",
                "--dither=diffusion",
                "--animate=on",
                "--speed=0.5",
                "--duration=inf",
                "--bg=#100100",
                "--clear",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap_or_else(|err| panic!("failed to spawn chafa: {err}"));

        let Some(stdout) = child.stdout.take() else {
            let _ = child.wait();
            return;
        };

        let mut reader = BufReader::new(stdout);
        let mut buf = [0_u8; 4096];
        let mut pending = String::new();
        let mut frame = String::new();

        loop {
            let read = match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };

            pending.push_str(&String::from_utf8_lossy(&buf[..read]));

            while let Some(marker) = next_marker_index(&pending) {
                let segment = pending[..marker].to_string();
                if !segment.trim().is_empty() {
                    frame.push_str(&segment);
                    if let Some(lines) = frame_to_lines(&frame) {
                        let _ = tx.send(lines);
                    }
                }

                frame.clear();
                pending.drain(..marker + marker_len(&pending[marker..]));
            }

            frame.push_str(&pending);
            pending.clear();
        }

        let _ = child.wait();
    });

    rx
}

pub fn hero_stream(width: u16, height: u16) -> Receiver<Vec<Line<'static>>> {
    spawn_chafa_stream(HERO_GIF_PATH, width, height)
}

pub fn hero_stream_initial_frame(width: u16, height: u16) -> Vec<Line<'static>> {
    render_frame(HERO_GIF_PATH, width, height)
}

fn frame_to_lines(frame: &str) -> Option<Vec<Line<'static>>> {
    let text = frame.to_string().into_text().ok()?;
    Some(text.lines)
}

fn next_marker_index(text: &str) -> Option<usize> {
    let a = text.find("\u{1b}[H");
    let b = text.find("\u{1b}[2J");
    match (a, b) {
        (Some(x), Some(y)) => Some(x.min(y)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

fn marker_len(rest: &str) -> usize {
    if rest.starts_with("\u{1b}[H") {
        3
    } else if rest.starts_with("\u{1b}[2J") {
        4
    } else {
        0
    }
}
