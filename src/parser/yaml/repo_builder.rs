use crate::model::Repository;
use crate::parser::yaml::file::Command;
use super::error::*;
use super::file::YamlFile;

fn build_repo(yaml: &str) -> Result<Repository> {
    let f: YamlFile = serde_yaml::from_str(yaml)?;
    let mut repo = Repository::default();

    for command in f.commands.iter() {
        match command {
            Command::Commit(c) => {
                repo.commit(c.name.clone());
            }
            Command::SimpleCommit(name) => {
                repo.commit(name.clone());
            }
            Command::Branch(b) => {
                if let Some(commit) = &b.at_commit {
                    repo.checkout_branch_at_commit(&b.name,commit);
                } else {
                    repo.checkout_branch(&b.name);
                }
            }
            Command::Merge(m) => {
                repo.merge(m.commit_name.clone(), &m.branches);
            }

        }
    }

    Ok(repo)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::parser::yaml::repo_builder::build_repo;

    #[test]
    fn three_commits() {
        // Setup
        let yaml = "
        commands: ['A','B','C']
        ";

        // Act
        let repo = build_repo(&yaml).unwrap();

        // Test
        assert_eq!(repo.branch_iter().count(), 1);
        assert_eq!(repo.branch_commits("main").len(), 3);
    }
}