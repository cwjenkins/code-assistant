// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ollama_rs::{
    /*generation::completion::{
        request::GenerationRequest, GenerationContext, GenerationResponseStream,
},*/
    generation::chat::{
	request::ChatMessageRequest, ChatMessage, ChatMessageResponse, MessageRole,
    },
    Ollama,
};
use tauri::Manager;
use tokio::sync::mpsc;
//use tokio::io::{stdout, AsyncWriteExt};
//use tokio_stream::StreamExt;
#[derive(Clone, serde::Serialize)]
struct AskResponse {
    message: String,
}

#[tauri::command]
async fn list_models() -> Vec<String> {
    let ollama = Ollama::default();

    let list_model_response = ollama.list_local_models().await.unwrap();

    return list_model_response.iter().map( |model| model.name.clone() ).collect();
}

#[tauri::command]
async fn ask_model(model: String, inquiry: String) -> String {
    println!("Model is: {model} and inquiry is: {inquiry}");
    let ollama = Ollama::default();
    let message = ChatMessage::new(MessageRole::User, inquiry.to_string());
    let request = ChatMessageRequest::new(model.to_string(), vec![message]);
    let response = ollama.send_chat_messages(request).await.unwrap();
    println!("Response: {}", response.message.content);
    return response.message.content;
}

fn emit_response(app: tauri::AppHandle) {
    app.emit_all("ask-model", AskResponse { message: "Hello".to_owned() });
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_models, ask_model])
        .run(tauri::generate_context!("../src-tauri/tauri.conf.json"))
        .expect("error while running tauri application");
}
