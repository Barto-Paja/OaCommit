pub struct BranchInfo{
    pub name: String,
    pub is_remote: bool,
}

impl BranchInfo {
    pub fn new (name: String, is_remote: bool) -> Self{
        BranchInfo{
            name,
            is_remote,
        }
    }

    pub fn new_local(name: String) -> Self{
        BranchInfo{
            name,
            is_remote: false,
        }
    }

    pub fn new_remote(name: String) -> Self{
        BranchInfo{
            name,
            is_remote: true,
        }
    }
}