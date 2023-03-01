#[derive(Debug)]
pub struct Commit {
    pub id: String,
    pub time: usize,
    pub branch: String,
    pub parents: Vec<String>,
}
