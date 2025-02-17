use git2::{Repository, Error, Oid, BranchType, Branch};
use serde::Serialize;

pub struct GitGate {
    pub path: String,
    pub commit: Option<Oid>,
    is_repository: bool,
    pub error_message: Option<String>,
}

impl GitGate {
    pub fn new() -> GitGate {
        GitGate{
            path: String::new(),
            commit: None,
            is_repository: false,
            error_message: None,
        }
    }

    pub fn init(&mut self) -> bool {
        dbg!("TUTAJ");
       match (Repository::init(self.path.clone())){
           Ok(r) => {
               dbg!("Sukces");
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

