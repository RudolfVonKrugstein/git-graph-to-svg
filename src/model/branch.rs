pub struct Branch {
    pub name: String,
    pub style: String,
    pub priority: usize,
    pub current_commit: Option<String>,
}