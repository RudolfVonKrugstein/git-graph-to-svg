use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CommitCommand {
    #[serde(rename = "name")]
    pub(crate) name: String
}

#[derive(Serialize, Deserialize)]
pub struct BranchCommand {
    #[serde(rename = "branch")]
    pub(crate) name: String,
    pub(crate) at_commit: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MergeCommand {
    #[serde(rename = "merge")]
    pub(crate) commit_name: String,
    pub(crate) branches: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Commit(CommitCommand),
    SimpleCommit(String),
    Branch(BranchCommand),
    Merge(MergeCommand)

}

#[derive(Serialize, Deserialize)]
pub(crate) struct YamlFile {
   pub(crate) commands: Vec<Command>
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        // Setup
        let yaml = "
          commands: []
          ";
        //Act
        let f: YamlFile = serde_yaml::from_str(yaml).unwrap();

        // Test
        assert_eq!(f.commands.len(), 0);
    }

}