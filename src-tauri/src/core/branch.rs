pub struct BranchInfo{
    pub name: String,
    pub is_remote: bool,
    pub is_head: bool,
}

impl BranchInfo {
    pub fn new (name: String, is_remote: bool, is_head: bool) -> Self{
        BranchInfo{
            name,
            is_remote,
            is_head,
        }
    }

    pub fn new_local(name: String) -> Self{
        BranchInfo{
            name,
            is_remote: false,
            is_head: false,
        }
    }

    pub fn new_local_head(name: String, is_head: bool) -> Self{
        BranchInfo{
            name,
            is_remote: false,
            is_head,
        }
    }

    pub fn new_remote(name: String) -> Self{
        BranchInfo{
            name,
            is_remote: true,
            is_head: false,
        }
    }
}