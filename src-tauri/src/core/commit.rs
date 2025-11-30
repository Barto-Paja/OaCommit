pub struct CommitInfo{
    pub id: String,
    pub message: String,
    pub author: String,
    pub timestamp: i64,
}

impl CommitInfo{
    pub fn new(id: String, message: String, author: String, timestamp: i64) -> CommitInfo{
        CommitInfo{
            id,
            message,
            author,
            timestamp,
        }
    }
}