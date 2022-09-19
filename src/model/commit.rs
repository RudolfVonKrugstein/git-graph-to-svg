pub struct Commit {
    // The id of the commit
    // Normally shown inside the commits circle
    id: String,
    // All parents of this commits (their ids)
    parents: Vec<String>,
    // The commit message
    message: String,
    // The branch of this commit belongs to
    // Of course in git a commit can belong to multiple branches.
    // Here the branch determines the drawing style of the commit
    // And its row
    branch: String,
    // The time of the commit
    // This is just a number used for ordering commits
    pub(crate) time: i32,
    // The style, may override the branch style
    style: Option<String>,
}

impl Commit {
    pub fn new(
        id: String,
        parents: Vec<String>,
        message: String,
        branch: String,
        time: i32,
    ) -> Commit {
        Commit {
            id,
            parents,
            message,
            branch,
            time,
            style: None,
        }
    }
}
