use crate::parser::instructions::Instruction;
use std::collections::{HashMap, HashSet};
use crate::model::branch::Branch;
use crate::model::commit::Commit;

pub struct Repository {
    pub branches: HashMap<String, Branch>,
    pub head: Option<String>,
    pub commits: HashMap<String, Commit>,
}

impl Repository {
    pub fn default() -> Repository {
        Repository {
            branches: HashMap::new(),
            head: None,
            commits: HashMap::new(),
        }
    }

    pub fn commit_iter(&self) -> impl Iterator<Item=&Commit> {
        self.commits.values()
    }

    pub fn branch_iter(&self) -> impl Iterator<Item=&Branch> {
        self.branches.values()
    }

    fn add_commit_and_history_to_hashset(&self, hash_set: &mut HashSet<String>,commit_id: &str) {
        if hash_set.insert(commit_id.to_string()) {
            for parent in self.commits.get(commit_id).unwrap().parents.iter() {
                self.add_commit_and_history_to_hashset(hash_set, parent);
            }
        }
    }

    pub fn branch_commits(&self, branch: &str) -> HashSet<String> {
        // Build a hash set of all commits in the branch
        let head = if let Some(commit) =self.branch_head(branch) {
            commit
        } else {
            return HashSet::new();
        };
        let mut commits = HashSet::new();
        self.add_commit_and_history_to_hashset(&mut commits, head);

        commits
    }

    pub fn apply_instruction(&mut self, i: &Instruction) {
        match i {
            Instruction::BRANCH(name, args) => {
                self.add_branch(name.clone(), None);
            }
            Instruction::COMMIT(id) => {
                self.commit(id.clone());
            }
            Instruction::CHECKOUT(branch) => {
                self.checkout_branch(branch);
            }
            Instruction::MERGE(commit_id, addition_parents) => {
                self.merge(commit_id.clone(), addition_parents.clone());
            }
        }
    }

    pub fn branch_head(&self, branch: &str) -> Option<&String> {
        self.branches
            .get(branch)
            .and_then(|b| b.current_commit.as_ref())
    }

    pub fn current_commit(&self) -> Option<&String> {
        self.head
            .as_ref()
            .and_then(|b| self.branch_head(b))
    }

    pub fn add_branch(&mut self, name: String, reference: Option<&String>) {
        let commit = if let Some(r) = reference {
           if self.commits.contains_key(r) {
               Some(r.to_string())
           } else {
               self.branch_head(r).cloned()
           }
        } else {
            self.current_commit().cloned()
        };
        self.branches.insert(
            name.clone(),
            Branch {
                name: name.clone(),
                style: "".to_string(),
                priority: self.branches.len(),
                current_commit: commit,
            },
        );
        // If its the only branch, do a checkout
        if self.head.is_none() {
            self.head = Some(name);
        }
    }

    pub fn checkout_branch(&mut self, name: &str) {
        if !self.branches.contains_key(name) {
            self.add_branch(name.to_string(), None);
        }
        self.head = Some(name.to_string());
    }

    pub fn checkout_branch_at_commit(&mut self, name: &str, commit: &str) {
        if !self.branches.contains_key(name) {
            self.add_branch(name.to_string(), Some(&commit.to_string()));
        }
        self.head = Some(name.to_string());
    }

    pub fn commit(&mut self, id: String) {
        self.merge(id, Vec::new());
    }

    pub fn merge(&mut self, id: String, add_branches: Vec<String>) {
        if self.head == None {
            self.checkout_branch(&"main".to_string());
        }

        if let Some(branch) = &self.head {
            // Collect the parent commits
            let mut parent_commits = Vec::new();
            // Of our current branch
            if let Some(branch_parent) = self.current_commit()
            {
                parent_commits.push(branch_parent.clone());
            }
            // And the additional branches
            for branch in &add_branches {
                // Is it a branch?
                if let Some(branch_commit) = self
                    .branches
                    .get(branch)
                    .and_then(|b| b.current_commit.as_ref())
                {
                    parent_commits.push(branch_commit.clone());
                }
                // Or just a commit?
                if self.commits.contains_key(branch) {
                    parent_commits.push(branch.clone());
                }
            }

            self.commits.insert(
                id.clone(),
                Commit {
                    id: id.clone(),
                    time: self.commits.len(),
                    branch: branch.clone(),
                    parents: parent_commits,
                },
            );
            self.branches.get_mut(branch).unwrap().current_commit = Some(id.clone());
        } else {
            panic!("no branch, cannot commit")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_default() {
        // Setup
        let repo = Repository::default();

        // Act

        // Test
        assert_eq!(repo.commit_iter().count(), 0);
        assert_eq!(repo.branch_iter().count(), 0);
    }

    #[test]
    fn one_commit_on_main() {
        // Setup
        let mut repo = Repository::default();

        // Act
        repo.commit("A".to_string());

        // Test
        assert_eq!(repo.commit_iter().count(), 1);
        assert_eq!(repo.branch_iter().count(), 1);
        assert_eq!(repo.branch_iter().collect::<Vec<&Branch>>().first().unwrap().name, "main");
    }

    #[test]
    fn multiple_branches() {
        // Setup
        let mut repo = Repository::default();

        // Act
        repo.commit("A".to_string());
        repo.checkout_branch("feature");
        repo.commit("B".to_string());
        repo.checkout_branch("main");
        repo.commit("C".to_string());

        // Test
        assert_eq!(repo.commit_iter().count(), 3);
        assert_eq!(repo.branch_iter().count(), 2);
        assert_eq!(repo.branch_head("main"), Some(&"C".to_string()));
        assert_eq!(repo.branch_commits("feature"), HashSet::from_iter(vec!["A".to_string(),"B".to_string()]));
    }

    #[test]
    fn separate_branches() {
        // Setup
        let mut repo = Repository::default();

        // Act
        repo.commit("A".to_string());
        repo.checkout_branch("feature1");
        repo.commit("B".to_string());
        repo.checkout_branch_at_commit("feature2", "A");
        repo.commit("C".to_string());
        repo.checkout_branch("main");
        repo.commit("D".to_string());

        // Test
        assert_eq!(repo.branch_commits("main"), HashSet::from_iter(vec!["A".to_string(),"D".to_string()]));
        assert_eq!(repo.branch_commits("feature1"), HashSet::from_iter(vec!["A".to_string(),"B".to_string()]));
        assert_eq!(repo.branch_commits("feature2"), HashSet::from_iter(vec!["A".to_string(),"C".to_string()]));
    }

    #[test]
    fn merge() {
        // Setup
        let mut repo = Repository::default();

        // Act
        repo.commit("A".to_string());
        repo.checkout_branch("feature1");
        repo.commit("B".to_string());
        repo.checkout_branch_at_commit("feature2", "A");
        repo.commit("C".to_string());
        repo.checkout_branch("main");
        repo.commit("D".to_string());
        repo.merge("M".to_string(), vec!["feature1".to_string()]);

        // Test
        assert_eq!(repo.branch_commits("main"), HashSet::from_iter(vec!["A".to_string(),"B".to_string(), "D".to_string(),"M".to_string()]));
        assert_eq!(repo.branch_commits("feature1"), HashSet::from_iter(vec!["A".to_string(),"B".to_string()]));
        assert_eq!(repo.branch_commits("feature2"), HashSet::from_iter(vec!["A".to_string(),"C".to_string()]));
    }

    #[test]
    fn merge_multiple_branches() {
        // Setup
        let mut repo = Repository::default();

        // Act
        repo.commit("A".to_string());
        repo.checkout_branch("feature1");
        repo.commit("B".to_string());
        repo.checkout_branch_at_commit("feature2", "A");
        repo.commit("C".to_string());
        repo.checkout_branch("main");
        repo.commit("D".to_string());
        repo.merge("M".to_string(), vec!["feature1".to_string(),"feature2".to_string()]);

        // Test
        assert_eq!(repo.branch_commits("main"), HashSet::from_iter(vec!["A".to_string(),"B".to_string(),"C".to_string(), "D".to_string(),"M".to_string()]));
        assert_eq!(repo.branch_commits("feature1"), HashSet::from_iter(vec!["A".to_string(),"B".to_string()]));
        assert_eq!(repo.branch_commits("feature2"), HashSet::from_iter(vec!["A".to_string(),"C".to_string()]));
    }
}