use serde::Serialize;
use crate::core::branch::BranchInfo as CoreBranch;

#[derive(Serialize)]
pub struct Branch {
    pub name: String,
    pub target: String,
    pub is_remote: bool,
    pub icon: String,
}

// impl From<&CoreBranch> for Branch {
//     fn from(b: &CoreBranch) -> Self {
//         Self {
//             name: b.name.clone(),
//             target: b.target.clone(),
//             is_remote: b.is_remote,
//             icon: if b.is_remote { "leaf" } else { "oak" }.into(),
//         }
//     }
// }
