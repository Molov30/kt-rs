use notify::{event::CreateKind, recommended_watcher, EventKind, Watcher};
use std::{ops::Add, path::Path};
use tauri::{Emitter, Manager};

use crate::state;

#[tauri::command]
pub async fn restart_service(state: tauri::State<'_, state::AppState>) -> Result<String, String> {
    let out = tokio::process::Command::new("systemctl")
        .args(["restart", &state.cfg.service_name])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    match out.status.success() {
        true => Ok("OK".to_string()),
        false => Err(String::from_utf8_lossy(&out.stderr).into_owned()),
    }
}

#[tauri::command]
pub async fn start_service(state: tauri::State<'_, state::AppState>) -> Result<String, String> {
    let out = tokio::process::Command::new("systemctl")
        .args(["start", &state.cfg.service_name])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    match out.status.success() {
        true => Ok("OK".to_string()),
        false => Err(String::from_utf8_lossy(&out.stderr).into_owned()),
    }
}

#[tauri::command]
pub async fn stop_service(state: tauri::State<'_, state::AppState>) -> Result<String, String> {
    let out = tokio::process::Command::new("systemctl")
        .args(["stop", &state.cfg.service_name])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    match out.status.success() {
        true => Ok("OK".to_string()),
        false => Err(String::from_utf8_lossy(&out.stderr).into_owned()),
    }
}

#[tauri::command]
pub async fn kill_ui(state: tauri::State<'_, state::AppState>) -> Result<String, String> {
    let pidof_out = tokio::process::Command::new("pidof")
        .arg(&state.cfg.client_ps_name)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !pidof_out.status.success() {
        return Err(String::from_utf8_lossy(&pidof_out.stderr)
            .add(":UI may not be running")
            .into_owned());
    }

    let ps_pid_cow = String::from_utf8_lossy(&pidof_out.stdout);
    let ps_pid = ps_pid_cow.trim();
    if ps_pid.is_empty() {
        return Err("UI process not found".into());
    }

    let out = tokio::process::Command::new("kill")
        .arg(ps_pid)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    match out.status.success() {
        true => Ok("OK".to_string()),
        false => Err(String::from_utf8_lossy(&out.stderr).into_owned()),
    }
}

#[tauri::command]
pub async fn enable_debug(state: tauri::State<'_, state::AppState>) -> Result<String, String> {
    let content = tokio::fs::read_to_string(&state.cfg.config_path)
        .await
        .map_err(|e| e.to_string())?;

    let content = content.replace("INFO", "DEBUG");
    write_file_safe(&state.cfg.config_path, content)
        .await
        .map_err(|e| e.to_string())?;

    Ok("OK".into())
}

#[tauri::command]
pub async fn disable_obsolete(state: tauri::State<'_, state::AppState>) -> Result<String, String> {
    let mut dir = tokio::fs::read_dir(state.cfg.dp_path)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(entry) = dir.next_entry().await.map_err(|e| e.to_string())? {
        let content = tokio::fs::read_to_string(entry.path())
            .await
            .map_err(|e| e.to_string())?;

        let content = content.replace(r"obsolete=true", "obsolete=false");
        write_file_safe(&entry.path().to_string_lossy(), content)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok("OK".into())
}

#[tauri::command]
pub async fn clear_logs(state: tauri::State<'_, state::AppState>) -> Result<String, String> {
    let mut dir = tokio::fs::read_dir(&state.cfg.logs_path)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(entry) = dir.next_entry().await.map_err(|e| e.to_string())? {
        let p = entry.path();
        if p.is_dir() {
            tokio::fs::remove_dir_all(&p)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            tokio::fs::remove_file(&p)
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    Ok("OK".into())
}

#[tauri::command]
pub async fn play_alert() {
    let wav = generate_beep_wav(880.0, 1.5, 0.85);
    let players: &[(&str, &[&str])] = &[
        ("pw-play", &["-"]),
        ("paplay", &["-"]),
        ("aplay", &["-q", "-"]),
    ];
    for (player, args) in players {
        use std::io::Write;
        use std::process::Stdio;
        let child = std::process::Command::new(player)
            .args(*args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        if let Ok(mut child) = child {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(&wav);
            }
            if child.wait().map(|s| s.success()).unwrap_or(false) {
                return;
            }
        }
    }
}

fn generate_beep_wav(freq: f32, duration_secs: f32, amplitude: f32) -> Vec<u8> {
    let sample_rate: u32 = 44100;
    let samples = (sample_rate as f32 * duration_secs) as u32;
    let data_size = samples * 2;

    let mut wav = Vec::with_capacity(44 + data_size as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());

    for i in 0..samples {
        let t = i as f32 / sample_rate as f32;
        let envelope = if t < 0.02 {
            t / 0.02
        } else if t > duration_secs - 0.1 {
            (duration_secs - t) / 0.1
        } else {
            1.0
        };
        let sample = (envelope
            * amplitude
            * (2.0 * std::f32::consts::PI * freq * t).sin()
            * i16::MAX as f32) as i16;
        wav.extend_from_slice(&sample.to_le_bytes());
    }

    wav
}

pub fn watch_dumps(app: tauri::AppHandle) {
    let state = app.state::<crate::state::AppState>();
    let watch_path = state.cfg.logs_path.to_string();

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = recommended_watcher(tx).unwrap();
        watcher
            .watch(
                std::path::Path::new(&watch_path),
                notify::RecursiveMode::NonRecursive,
            )
            .unwrap();

        for res in rx {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Create(CreateKind::File)) {
                    for path in event.paths {
                        if path
                            .file_name()
                            .map_or(false, |e| e.to_string_lossy().contains("dmp"))
                        {
                            let _ = app.emit("new-dump", path.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    });
}

async fn write_file_safe(path: &str, content: String) -> Result<String, String> {
    let orig_path = Path::new(path);
    let tmp_path = orig_path.with_extension("tmp");
    tokio::fs::write(&tmp_path, content)
        .await
        .map_err(|e| e.to_string())?;

    tokio::fs::rename(tmp_path, orig_path)
        .await
        .map_err(|e| e.to_string())?;

    Ok("OK".into())
}
