use serde::Serialize;
use crate::core::commit::CommitInfo as CoreCommit;

#[derive(Serialize)]
pub struct Commit{
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
    pub branches: Vec<String>,
    pub parents: Vec<String>,
    pub is_head: bool,
    pub icon: String,
}

// impl From<&CoreCommit> for Commit {
//     fn from(c: &CoreCommit) -> Self {
//         Self {
//             hash: c.hash.clone(),
//             message: c.message.clone(),
//             author: c.author.clone(),
//             date: format!("{}", chrono::NaiveDateTime::from_timestamp(c.date,0)),
//             branches: c.branches.clone(),
//             parents: c.parents.clone(),
//             is_head: false, // ustawisz później w mappingu z branch info
//             icon: "leaf".into(),
//         }
//     }
// }