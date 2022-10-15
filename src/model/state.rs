use std::cmp::Ordering;
use std::collections::HashMap;
use crate::parser::instructions::Instruction;

pub struct Branch {
    pub name: String,
    pub style: String,
    pub priority: usize,
    pub current_commit: Option<String>,
}

#[derive(Debug)]
pub struct Commit {
    pub id: String,
    pub time: usize,
    pub branch: String,
    pub parents: Vec<String>,
}

pub struct ParseState {
    pub branches: HashMap<String, Branch>,
    pub current_branch: Option<String>,
    pub commits: HashMap<String, Commit>
}

impl ParseState {
    pub fn new() -> ParseState {
        ParseState{
            branches: HashMap::new(),
            current_branch: None,
            commits: HashMap::new(),
        }
    }

    pub fn apply_instruction(&mut self, i: &Instruction) {
        match i {
            Instruction::BRANCH(name, args) => {
               self.add_branch(name);
            }
            Instruction::COMMIT(id) => {
                self.commit(id);
            }
            Instruction::CHECKOUT(branch) => {
                self.switch_branch(branch);
            }
            Instruction::MERGE(commit_id, addition_parents) => {
                self.merge(commit_id, addition_parents.clone());
            }
        }
    }

    pub fn branch_head(&self, branch: &String) -> Option<&String> {
        self.branches.get(branch).and_then(
            |b| b.current_commit.as_ref(),
        )
    }

    pub fn current_commit(&self) -> Option<&String> {
        self.current_branch.as_ref().and_then(
            |b| self.branch_head(b)
        )
    }

    pub fn add_branch(&mut self, name: &String) {
        self.branches.insert(name.clone(),Branch {
            name: name.clone(),
            style: "".to_string(),
            priority: self.branches.len(),
            current_commit: self.current_commit().cloned(),
        });
        // If its the only branch, do a checkout
        if self.current_branch == None {
            self.current_branch  = Some(name.clone());
        }
    }

    pub fn switch_branch(&mut self, name: &String) {
        if !self.branches.contains_key(name) {
            self.add_branch(name);
        }
        self.current_branch  = Some(name.clone());
    }

    pub fn commit(&mut self, id: &String) {
        self.merge(id, Vec::new());
    }

    pub fn merge(&mut self, id: &String, add_branches: Vec<String>) {
        if self.current_branch == None {
            self.switch_branch(&"main".to_string());
        }


        if let Some(branch) = &self.current_branch {
            // Collect the parent commits
            let mut parent_commits = Vec::new();
            // Of our current branch
            if let Some(branch_parent) = self.branches.get(branch).and_then(|b| b.current_commit.as_ref()) {
                parent_commits.push(branch_parent.clone());
            }
            // And the additional branches
            for branch in &add_branches {
                // Is it a branch?
                if let Some(branch_commit) = self.branches.get(branch).and_then(|b| b.current_commit.as_ref()) {
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
                }
            );
            self.branches.get_mut(branch).unwrap().current_commit = Some(id.clone());
        } else {
            panic!("no branch, cannot commit")
        }
    }
}

