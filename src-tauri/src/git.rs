use std::path::Path;
use git2::{Repository, Error, Oid, BranchType, Branch};
use serde::Serialize;
use std::collections::HashMap;
use std::ptr::hash;
use std::sync::Arc;
use crate::core::repo_state::RepositoryState as CoreRepoState;
use crate::core::commit::CommitInfo as CoreCommitInfo;
use crate::core::branch::BranchInfo as CoreBranchInfo;
use crate::{get_branches};
use crate::view_models::branch::Branch as BranchView;
use crate::view_models::commit::Commit as CommitView;


pub struct GitGate {
    pub path: String,
    pub commit: Option<Oid>,
    is_repository: bool,
    pub error_message: Option<String>,
    pub url: String,

    pub state: CoreRepoState,
}

impl GitGate {
    pub fn new() -> GitGate {
        GitGate{
            path: String::new(),
            commit: None,
            is_repository: false,
            error_message: None,
            url: String::new(),
            state: CoreRepoState::new(),
        }
    }

    pub fn refresh(&mut self) -> bool {
        _ = self.git_log();
        self.get_branches()
    }
    fn git_log(&mut self) -> bool {

        let repository = match Repository::open(&self.path) {
            Ok(r) => r,
            Err(e) => {
                self.error_message = Some(format!("{}", e));
                return false
            },
        };

        let mut revwalk = match repository.revwalk() {
            Ok(rw) => rw,
            Err(e) => {
                self.error_message = Some(format!("{}", e));
                return false
            }
        };

        if let Err(e) = revwalk.push_head() {
            self.error_message = Some(format!("{}", e));
            return false;
        }

        let mut commits = Vec::new();

        for oid in revwalk {
            let oid = match oid {
                Ok(oid) => oid,
                Err(e) => {
                    self.error_message = Some(format!("{}", e));
                    return false;
                }
            };

            let commit = match repository.find_commit(oid){
                Ok(commit) => commit,
                Err(e) => {
                    self.error_message = Some(format!("{}", e));
                    return false;
                }
            };

            let summary = commit.summary().unwrap_or("No message").to_string();
            let hash = commit.id().to_string()[0..12].to_string();

            let summary = commit.summary().unwrap_or("No message").to_string();
            let hash = commit.id().to_string()[0..12].to_string();
            let author = commit.author().name().unwrap_or("Unknown").to_string();
            let date = commit.time().seconds();

            let parents = commit
                .parents()
                .map(|p| p.id().to_string())
                .collect::<Vec<_>>();

            let mut commit_vm = CoreCommitInfo {
                id: hash,
                timestamp: date,
                author: author,
                message: summary,
            };

            commits.push(commit_vm);

        }

        self.state.commits = commits;
        return true;
    }

    fn get_branches(&mut self) -> bool {
        let repository = match Repository::open(&self.path) {
            Ok(r) => r,
            Err(e) => {
                self.error_message = Some(format!("{}", e));
                return false
            },
        };

        let mut branches = Vec::new();

        if let Ok(local_branches) = repository.branches(Some(BranchType::Local)) {
            for branch in local_branches.flatten() {
                if let Ok(Some(name)) = branch.0.name() {
                    branches.push(CoreBranchInfo {
                        name: name.to_string(),
                        is_remote: false,
                    });
                }
            }
        }
        else{
            // ToDo: Message Error tu ma być
            return false;
        }

        if let Ok(remote_branches) = repository.branches(Some(BranchType::Remote)) {
            for branch in remote_branches.flatten() {
                if let Ok(Some(name)) = branch.0.name() {
                    branches.push(CoreBranchInfo {
                        name: name.to_string(),
                        is_remote: true,
                    });
                }
            }
        }
        else{
            // ToDo: Message Error tu ma być
            return false;
        }

        self.state.branches = branches;

        true
    }

    pub fn init(&mut self) -> bool {
       match (Repository::init(self.path.clone())){
           Ok(r) => {
               self.is_repository = true;
               true
           },
           Err(error) => {
               self.error_message = Some(format!("Init repository failed: {}", error));
               false
           }
       }
    }

    pub fn clone(&mut self) -> bool {
        let path = Path::new(&self.path);
        match git2::build::RepoBuilder::new().clone(self.url.clone().as_str(), path) {
            Ok(r) => {
                self.is_repository = true;
                true
            },
            Err(error) => {
                self.error_message = Some(format!("Init repository failed: {}", error));
                false
            }
        }
    }
}

