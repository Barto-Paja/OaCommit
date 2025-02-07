use git2::{Repository, Error, Oid, BranchType, Branch};
use serde::Serialize;
use std::collections::HashMap;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: i64,
    pub branch: String,
    pub parents: Vec<String>
}

#[tauri::command]
fn get_git_log(repo_path: String) -> Result<Vec<Commit>, String> {
    let repo = Repository::open(&repo_path).map_err(|e| e.to_string())?;
    let mut revwalk = repo.revwalk().map_err(|e| e.to_string())?;
    revwalk.push_head().map_err(|e| e.to_string())?;

    let mut commits = Vec::new();

    for oid in revwalk {
        let oid = oid.map_err(|e| e.to_string())?;
        let commit = repo.find_commit(oid).map_err(|e| e.to_string())?;

        let summary = commit.summary().unwrap_or("No message").to_string();
        let hash = commit.id().to_string()[0..12].to_string();
        let author = commit.author().name().unwrap_or("Unknown").to_string();
        let date = commit.time().seconds(); // Timestamp commita

        let mut parents = Vec::new();
        for parent in commit.parents() {
            parents.push(parent.id().to_string());
        }

        commits.push(Commit {
            hash: hash.clone(),
            message: summary.clone(),
            author: author.clone(),
            date: date.clone(),
            branch: String::new(),
            parents: parents,
        });
    }

    Ok(commits)
}

#[cfg_attr(mobile, tauri::mobile_entry_point, tauri::tauri_plugin_dialog, tauri::tauri_plugin_fs)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_git_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
