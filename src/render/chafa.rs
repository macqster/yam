use std::{
    io::{BufReader, Read},
    process::Command,
    sync::mpsc::{self, Receiver},
    thread,
};

use ansi_to_tui::IntoText;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use ratatui::text::Line;

const HERO_GIF_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif");

pub fn render_frame(path: &str, width: u16, height: u16) -> Vec<Line<'static>> {
    let size_arg = format!("{}x{}", width, height);
    // NOTE: snapshot helper MUST be static; animation is handled by the streaming pipeline
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
        .arg("--fg-only")
        .arg("--bg=#100100")
        .arg("--animate=off")
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
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: height,
                cols: width,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap_or_else(|err| panic!("failed to open pty: {err}"));

        let mut cmd = CommandBuilder::new("chafa");
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        cmd.arg("--probe=off");
        cmd.arg(&path);
        cmd.arg("--size");
        cmd.arg(&size_arg);
        cmd.arg("--format=symbols");
        cmd.arg("--symbols=braille");
        cmd.arg("--colors=full");
        cmd.arg("--color-space=din99d");
        cmd.arg("--color-extractor=median");
        cmd.arg("--dither=diffusion");
        cmd.arg("--animate=on");
        cmd.arg("--speed=0.2");
        cmd.arg("--duration=inf");
        cmd.arg("--bg=#100100");
        cmd.arg("--clear");
        cmd.arg("--passthrough=screen");
        cmd.arg("--optimize=0");
        cmd.arg("--relative=off");

        let mut child = pair
            .slave
            .spawn_command(cmd)
            .unwrap_or_else(|err| panic!("failed to spawn chafa in pty: {err}"));
        drop(pair.slave);

        let stdout = pair
            .master
            .try_clone_reader()
            .unwrap_or_else(|err| panic!("failed to clone pty reader: {err}"));

        let mut reader = BufReader::new(stdout);
        let mut buf = [0_u8; 4096];
        let mut pending = String::new();
        let mut last_signature = String::new();

        loop {
            let read = match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            pending.push_str(&String::from_utf8_lossy(&buf[..read]));

            if pending.len() > 65_536 {
                trim_pending(&mut pending, 32_768);
            }

            if let Some(lines) = frame_from_stream(&pending, height) {
                let signature = frame_signature(&lines);
                if signature != last_signature {
                    let _ = tx.send(lines);
                    last_signature = signature;
                }
            }
        }

        if let Some(lines) = frame_from_stream(&pending, height) {
            let _ = tx.send(lines);
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

fn frame_from_stream(stream: &str, height: u16) -> Option<Vec<Line<'static>>> {
    let sanitized = sanitize_for_text(stream);
    let text = sanitized.into_text().ok()?;
    let mut lines: Vec<Line<'static>> = text
        .lines
        .into_iter()
        .filter(|line| {
            let s = line
                .spans
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<String>();
            !s.trim().is_empty()
        })
        .collect();
    let needed = height as usize;
    if lines.len() < needed {
        None
    } else {
        lines = lines.split_off(lines.len() - needed);
        Some(lines)
    }
}

fn frame_signature(lines: &[Line<'static>]) -> String {
    lines
        .iter()
        .map(|line| {
            line.spans
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn trim_pending(pending: &mut String, keep_chars: usize) {
    if pending.len() <= keep_chars {
        return;
    }
    let drop = pending.len().saturating_sub(keep_chars);
    let start = pending
        .char_indices()
        .find(|(idx, _)| *idx >= drop)
        .map(|(idx, _)| idx)
        .unwrap_or(0);
    pending.drain(..start);
}

fn sanitize_for_text(frame: &str) -> String {
    let mut out = String::with_capacity(frame.len());
    let mut chars = frame.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\u{1b}' {
            out.push(ch);
            continue;
        }

        let Some(next) = chars.next() else {
            break;
        };

        match next {
            '[' => {
                let mut seq = String::from("\u{1b}[");
                for c in chars.by_ref() {
                    seq.push(c);
                    if ('@'..='~').contains(&c) {
                        if c == 'm' {
                            out.push_str(&seq);
                        }
                        break;
                    }
                }
            }
            ']' => {
                while let Some(c) = chars.next() {
                    if c == '\u{7}' {
                        break;
                    }
                    if c == '\u{1b}' {
                        if matches!(chars.peek(), Some('\\')) {
                            chars.next();
                        }
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    out
}
