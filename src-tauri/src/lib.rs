mod git;
use crate::git::GitGate;

use git2::{Repository, Error, Oid, BranchType, Branch};
use serde::Serialize;
use std::collections::HashMap;
use std::ptr::hash;

use tokio::sync::Mutex;
use std::sync::Arc;
use tauri::State;

#[derive(Serialize)]
struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: i64,
    pub branches: Vec<String>,
    pub parents: Vec<String>
}

#[tauri::command]
fn get_remote_branches(repo_path: String) -> Vec<(String,String)> {
    let repo = Repository::open(&repo_path).map_err(|e| e.to_string()).unwrap();
    let mut mapped_branches : Vec<(String,String)> = Vec::new();

    if let Ok(branches) = repo.branches(Some(BranchType::Remote)) {
        for branch in branches.flatten() {
            if let (Ok(Some(branch_name)), Some(target)) = (branch.0.name(), branch.0.get().target()) {
                mapped_branches.push((target.to_string(), branch_name.to_string()));
            }
        }
    }

    mapped_branches
}

#[tauri::command]
fn get_local_branches(repo_path: String) -> Vec<(String,String)> {
    let repo = Repository::open(&repo_path).map_err(|e| e.to_string()).unwrap();
    let mut mapped_branches : Vec<(String,String)> = Vec::new();

    if let Ok(branches) = repo.branches(Some(BranchType::Local)) {
        for branch in branches.flatten() {
            if let (Ok(Some(branch_name)), Some(target)) = (branch.0.name(), branch.0.get().target()) {
                mapped_branches.push((target.to_string(), branch_name.to_string()));
            }
        }
    }

    mapped_branches
}

fn get_branches(repo: &Repository) -> Vec<(String,String)> {
    let mut mapped_branches : Vec<(String,String)> = Vec::new();

    if let Ok(branches) = repo.branches(None) {
        for branch in branches.flatten() {
            if let (Ok(Some(branch_name)), Some(target)) = (branch.0.name(), branch.0.get().target()) {
                mapped_branches.push((target.to_string(), branch_name.to_string()));
            }
        }
    }

    mapped_branches
}

#[tauri::command]
fn get_git_log(repo_path: String) -> Result<Vec<Commit>, String> {
    let repo = Repository::open(&repo_path).map_err(|e| e.to_string())?;
    let mut revwalk = repo.revwalk().map_err(|e| e.to_string())?;
    revwalk.push_head().map_err(|e| e.to_string())?;

    let mut commits = Vec::new();

    let branches = get_branches(&repo);

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

        let mut commit = Commit {
            hash: hash.clone(),
            message: summary.clone(),
            author: author.clone(),
            date: date.clone(),
            branches: Vec::new(),
            parents: parents,
        };

        for branch in &branches {
            let (commit_hash, name) = branch;
            if oid.to_string().eq(commit_hash) {
                commit.branches.push(name.clone());
            }
        }

        commits.push(commit);
    }

    Ok(commits)
}

fn extract_repo_name(url: &str) -> Option<String> {
    let url = url.trim_end_matches(".git"); // Usuń ".git" na końcu, jeśli istnieje
    let parts: Vec<&str> = url.rsplit('/').collect(); // Podziel URL od końca

    parts.first().map(|s| s.to_string()) // Pobierz ostatni element, jeśli istnieje
}


#[tauri::command]
async fn git_init(state: State<'_, Arc<Mutex<git::GitGate>>>, repo_path: String) -> Result<(), String> {
    let mut git_gate = state.lock().await;
    git_gate.path = repo_path;
    if git_gate.init() {
        Ok(())
    } else {
        Err(String::from("Failed to initialize git repo"))
    }
}

#[tauri::command]
async fn git_clone(state: State<'_, Arc<Mutex<git::GitGate>>>, repo_path: String, url: String) -> Result<(), String> {
    let mut git_gate = state.lock().await;
    dbg!(&repo_path);
    dbg!(&url);
    git_gate.path = String::from(format!("{}/{}",repo_path,extract_repo_name(&url).unwrap_or(String::new())));
    dbg!(&git_gate.path);
    git_gate.url = url;
    if git_gate.clone() {
        Ok(())
    } else {
        Err(String::from("Failed to clone git repo"))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point, tauri::tauri_plugin_dialog, tauri::tauri_plugin_fs)]
pub fn run() {
    let git_gate = Arc::new(Mutex::new(git::GitGate::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(git_gate.clone())
        .invoke_handler(tauri::generate_handler![get_git_log, get_remote_branches, get_local_branches,
            git_init, git_clone])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
