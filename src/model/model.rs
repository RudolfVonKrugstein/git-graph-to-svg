use super::branch::Branch;
use super::commit::Commit;
use super::style::Style;
use std::cmp::{max, Ordering};
use std::collections::HashMap;

pub struct Model {
    // All commits
    pub(crate) commits: HashMap<String, Commit>,
    // All branches
    pub(crate) branches: HashMap<String, Branch>,
    // Commit order
    commit_order: Vec<String>,
    // In which column are branches placed?
    branch_order: Vec<String>,
    // Styles
    styles: HashMap<String, Style>,
}

fn insert_object_into_sorted_list<F>(list: &mut Vec<String>, id: String, compare: F)
where
    F: Fn(&String, &String) -> Ordering,
{
    for (index, otherId) in list.iter().enumerate() {
        if compare(otherId, &id) != Ordering::Less {
            list.insert(index, id);
            return;
        }
    }
    // Not inserted, need to do that now
    list.push(id);
}

impl Model {
    pub(crate) fn new() -> Model {
        Model {
            commits: HashMap::new(),
            branches: HashMap::new(),
            commit_order: vec![],
            branch_order: vec![],
            styles: HashMap::new(),
        }
    }

    fn max_branch_priority(&self) -> i32 {
        self.branches
            .values()
            .fold(-1, |base, b| max(base, b.priority))
    }

    fn max_commit_time(&self) -> i32 {
        self.commits.values().fold(-1, |base, c| max(base, c.time))
    }

    pub(crate) fn get_branch_last_commit(&self, branch_name: &String) -> Option<&String> {
        self.branches
            .get(branch_name)
            .and_then(|b| b.commits.last())
    }

    pub(crate) fn add_branch(&mut self, name: String, style: String) {
        self.branches.insert(
            name.clone(),
            Branch::new(name.clone(), style, self.max_branch_priority() + 1),
        );
        insert_object_into_sorted_list(&mut self.branch_order, name.clone(), |a, b| {
            self.branches[a].priority.cmp(&self.branches[b].priority)
        });
    }

    pub(crate) fn add_commit(
        &mut self,
        id: String,
        branch: String,
        message: String,
        parents: Vec<String>,
    ) -> Result<(), String> {
        let commit_time = self.max_commit_time() + 1;
        self.commits.insert(
            id.clone(),
            Commit::new(id.clone(), parents, message, branch.clone(), commit_time),
        );
        // Put the commit into the list of ordered commits
        insert_object_into_sorted_list(&mut self.commit_order, id.clone(), |a, b| {
            self.commits[a].time.cmp(&self.commits[b].time)
        });

        // Put the commit into the list of comits of that branch!
        let branch_object = self.branches.get_mut(&branch).ok_or("Branch not found")?;
        insert_object_into_sorted_list(&mut branch_object.commits, id.clone(), |a, b| {
            self.commits[a].time.cmp(&self.commits[b].time)
        });

        Ok(())
    }

    fn branches_overlap(&self, first: &String, second: &String) -> bool {
        let branch1 = match self.branches.get(first) {
            None => return false,
            Some(b) => b,
        };
        let branch2 = match self.branches.get(second) {
            None => return false,
            Some(b) => b,
        };
        if branch1.commits.is_empty() || branch2.commits.is_empty() {
            return false;
        }
        let branch1_range = (
            branch1.commits.first().unwrap(),
            branch1.commits.last().unwrap(),
        );
        let branch2_range = (
            branch2.commits.first().unwrap(),
            branch2.commits.last().unwrap(),
        );
        if self.commits[branch1_range.0].time > self.commits[branch2_range.1].time {
            return false;
        }
        if self.commits[branch2_range.0].time > self.commits[branch1_range.1].time {
            return false;
        }
        return true;
    }

    fn calc_branch_columns(&self) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let current_col = 0;
        for branch in self.branch_order.iter() {
            let mut branch_inserted = false;
            for (test_col, col_branches) in result.iter_mut().enumerate() {
                let mut overlap = false;
                for col_branch in col_branches.iter() {
                    if self.branches_overlap(branch, col_branch) {
                        overlap = true;
                        break;
                    }
                }
                if !overlap {
                    col_branches.push(branch.clone());
                    branch_inserted = true;
                    break;
                }
            }
            if !branch_inserted {
                result.push(vec![branch.clone()]);
            }
        }
        result
    }
}
