// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ollama_rs::{
    generation::completion::{
        request::GenerationRequest, GenerationContext, GenerationResponseStream,
    },
    Ollama,
};
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;

#[derive(Clone, serde::Serialize)]
struct AskResponse {
    message: String,
}

#[derive(Clone, serde::Serialize)]
struct ListResponse {
    models: Vec<String>
}

static GLOBAL_APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_models() -> Vec<String> {
    let ollama = Ollama::default();
    
}

#[tauri::command]
fn ask_model(content: &str) -> String {
    let ollama = Ollama::default();
    
}

fn emit_response(app: tauri::AppHandle) {
    app.emit_all("ask-model", AskResponse { message: "" })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(|_app_handle, event| match event {
	    tauri::RunEvent::Ready => {
		println!("Window loaded");
		GLOBAL_APP_HANDLE.set(_app_handle.clone()).expect("Failed to set app handle");
	    }
	    _ => {}
	})
        .expect("error while running tauri application");
}
