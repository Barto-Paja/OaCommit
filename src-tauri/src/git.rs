use std::path::Path;
use git2::{Repository, Error, Oid, BranchType, Branch};
use serde::Serialize;

pub struct GitGate {
    pub path: String,
    pub commit: Option<Oid>,
    is_repository: bool,
    pub error_message: Option<String>,
    pub url: String,
}

impl GitGate {
    pub fn new() -> GitGate {
        GitGate{
            path: String::new(),
            commit: None,
            is_repository: false,
            error_message: None,
            url: String::new(),
        }
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

