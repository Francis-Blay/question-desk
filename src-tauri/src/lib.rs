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
use anthropic::types::ContentBlock;
use serde_json::json;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_question,
            list_questions,
            delete_question,
            draft_answer
        ])
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

#[derive(serde::Deserialize)]
struct MessagesResponse {
    content: Vec<ContentBlock>,
}

fn path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
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
) -> Result<Question, String> {
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
    read(&app)
}

#[tauri::command]
async fn draft_answer(app: tauri::AppHandle, id: String) -> Result<Question, String> {
    let key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| "ANTHROPIC_API_KEY is not set — see README".to_string())?;
    let model = "claude-opus-5-5";
    let max_tokens = 1024;

    let mut questions = read(&app)?;
    let target = questions
        .iter_mut()
        .find(|q| q.id == id)
        .ok_or_else(|| format!("no question with id {id}"))?;

    let client = reqwest::Client::builder()
        .no_proxy()
        .build()
        .map_err(|e| e.to_string())?;
    let post_data = json!({
        "model": model,
        "max_tokens": max_tokens,
        "messages": [{
            "role": "user",
            "content": target.question
        }]
    });

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&post_data)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    let raw_body = response.text().await.map_err(|e| e.to_string())?;

    println!("Anthropic API status: {status}");
    println!("Anthropic API raw response: {raw_body}");

    if !status.is_success() {
        return Err(format!("Anthropic API error ({status}): {raw_body}"));
    }

    let parsed: MessagesResponse = serde_json::from_str(&raw_body)
        .map_err(|e| format!("failed to parse response: {e} — raw body: {raw_body}"))?;

    let answer_text = parsed
        .content
        .into_iter()
        .find_map(|block| match block {
            ContentBlock::Text { text } => Some(text),
            _ => None,
        })
        .ok_or_else(|| "no text block in response".to_string())?;

    target.draft = Some(Draft {
        draft: answer_text,
        verify: vec!["needs review".to_string()],
    });

    let updated = target.clone();
    write(&app, &questions)?;
    Ok(updated)

    // 1. load the question by id 2. POST to /v1/messages 3. parse JSON into Draft
    // 4. write it back to the store 5. return the updated record
}