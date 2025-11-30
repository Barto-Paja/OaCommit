use CommitInfo;
use BranchInfo;

pub struct RepositoryState{
    pub commits: Vec<CommitInfo>,
    pub branches: Vec<BranchInfo>,
    pub current_branch: Option<String>,
}

impl RepositoryState {
    pub fn new() -> RepositoryState {
        RepositoryState {
            commits: Vec::new(),
            branches: Vec::new(),
            current_branch: None,
        }
    }
}