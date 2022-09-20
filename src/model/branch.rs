pub struct Branch {
    name: String,
    style: String,
    // The way branches are ordered is priority
    pub priority: i32,
    // The commits belonging to this branch (in order!)
    pub(crate) commits: Vec<String>,
    // The commit the branch is pointed at (if at all)
    current_commit: Option<String>,
}

impl Branch {
    pub(crate) fn new(name: String, style: String, priority: i32) -> Branch {
        Branch {
            name,
            style,
            priority,
            commits: Vec::new(),
            current_commit: None,
        }
    }

    pub(crate) fn current_commit(&self) -> Option<&String> {
        self.current_commit.as_ref().or(self.commits.last())
    }
}
