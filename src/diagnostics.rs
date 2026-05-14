use chrono::Local;
use serde_json::{json, Map, Value};
use std::{env, fs, io::Write, path::PathBuf, process, sync::OnceLock};

static SESSION_ID: OnceLock<Option<String>> = OnceLock::new();

fn diagnostics_enabled() -> bool {
    match env::var("YAM_DIAGNOSTICS") {
        Ok(value) => {
            let lowered = value.trim().to_ascii_lowercase();
            !(lowered.is_empty() || lowered == "0" || lowered == "false" || lowered == "off")
        }
        Err(_) => false,
    }
}

fn diagnostics_path() -> Option<PathBuf> {
    if !diagnostics_enabled() {
        return None;
    }

    if let Ok(path) = env::var("YAM_DIAGNOSTICS_PATH") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }

    if let Ok(state_home) = env::var("XDG_STATE_HOME") {
        return Some(PathBuf::from(state_home).join("yam/diagnostics.ndjson"));
    }

    env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".local/state/yam/diagnostics.ndjson"))
}

fn session_id() -> Option<&'static str> {
    SESSION_ID
        .get_or_init(|| {
            if !diagnostics_enabled() {
                return None;
            }

            if let Ok(existing) = env::var("YAM_DIAGNOSTICS_SESSION") {
                let trimmed = existing.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }

            Some(format!(
                "{}-{}",
                Local::now().format("%Y%m%dT%H%M%S%.3f%:z"),
                process::id()
            ))
        })
        .as_deref()
}

pub fn append_event(kind: &str, fields: &[(&str, Value)]) {
    let Some(path) = diagnostics_path() else {
        return;
    };
    let Some(session) = session_id() else {
        return;
    };

    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            return;
        }
    }

    let mut payload = Map::new();
    payload.insert("ts".to_string(), json!(Local::now().to_rfc3339()));
    payload.insert("session".to_string(), json!(session));
    payload.insert("pid".to_string(), json!(process::id()));
    payload.insert("kind".to_string(), json!(kind));
    for (key, value) in fields {
        payload.insert((*key).to_string(), value.clone());
    }

    let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };

    let Ok(line) = serde_json::to_string(&Value::Object(payload)) else {
        return;
    };

    let _ = writeln!(file, "{line}");
}
