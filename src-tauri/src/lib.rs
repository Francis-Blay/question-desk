// Question Desk — scaffold only.
//
// Build your Tauri commands here (M2: save_question, list_questions,
// delete_question — M4: draft_answer). Register each one in the
// invoke_handler below as you add it. See:
// https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![    
            save_question,
            list_questions,
            delete_question])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Draft {
    draft: String,
    verify: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Question {
    id: String,
    asked_at: String,
    asker: String,
    context: String,
    question: String,
    tags: Vec<String>,
    draft: Option<Draft>,
}

fn path(app: &tauri::AppHandle)->Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("could not resolve app data dir: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| format!("could not create app data dir: {e}"))?;
    Ok(dir.join("questions.json"))
}

fn write(app: &tauri::AppHandle, questions: &[Question]) -> Result<(), String> {
    let path = path(app)?;
    let raw = serde_json::to_string_pretty(questions)
        .map_err(|e| format!("could not serialize: {e}"))?;
    fs::write(&path, raw).map_err(|e| format!("could not write store: {e}"))
}

fn read(app: &tauri::AppHandle) -> Result<Vec<Question>, String> {
    let path = path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("could not read store: {e}"))?;
    serde_json::from_str(&raw).map_err(|e| format!("could not parse store: {e}"))
}

#[tauri::command]
fn save_question(
    app: tauri::AppHandle,
    asker: String,
    context: String,
    question: String,
    tags: Vec<String>,
) -> Result<Question, String>{
    let mut all = read(&app)?;
    let now = chrono::Utc::now();
    let id = format!("q_{}", now.format("%Y%m%d_%H%M%S"));

    let record = Question {
        id,
        asked_at: now.to_rfc3339(),
        asker,
        context,
        question,
        tags,
        draft: None,
    };

    all.push(record.clone());
    write(&app, &all)?;
    Ok(record)
}

#[tauri::command]
fn delete_question(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let mut all = read(&app)?;
    let before = all.len();
    all.retain(|q| q.id != id);
    if all.len() == before {
        return Err(format!("no question found with id {id}"));
    }
    write(&app, &all)?;
    Ok(())
}

#[tauri::command]
fn list_questions(app: tauri::AppHandle) -> Result<Vec<Question>, String> {
    let all = read(&app)?;
    Ok(all)
}