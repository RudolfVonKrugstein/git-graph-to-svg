use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::model::state::ParseState;

pub struct Commit {
    pub id: String,
    pub message: String,
    pub hash: String,
    pub parents: Vec<Parent>,
    pub style: String,
    pub time: usize,
}

pub struct Parent {
    pub commit: Rc<Commit>,
    pub style: String,
    pub ends_lane: bool,
    pub begins_lane: bool,
    pub in_lane: bool,
}

pub struct  Branch {
    pub name: String,
    pub head: Option<Rc<Commit>>,
    pub style: String
}

pub struct Lane {
    pub branch_names: Vec<String>,
    pub commits: Vec<Rc<Commit>>,
    pub col: usize,
}

// We temporary need the lane wit commit id information
// Because the commits have not been created when this lane is created
struct LaneWithCommitIds<'a> {
    pub branch_names: Vec<String>,
    pub commit_ids: Vec<&'a String>,
    pub priority: usize,
}

pub struct View {
    pub commits: HashMap<String, Rc<Commit>>,
    pub lanes: Vec<Rc<Lane>>,
    pub branches: HashMap<String, Rc<Branch>>,
    pub commits_branch_heads: HashMap<String, Vec<Rc<Branch>>>
}

impl View {
    pub fn from_state(state: &ParseState) -> View {
        // Time list of commit ids
        // We need this, because if we create commits like this
        // we ensure that the parents have already been created.
        let empty_id = "".to_string();
        let mut time_ord_commits = vec![&empty_id; state.commits.len()];
        for (id, commit) in state.commits.iter() {
            time_ord_commits[commit.time] = id;
        }
        // Make temporary lanes, which only use commit ids
        // because the commits have not been created yet
        let mut commit_id_lanes: Vec<LaneWithCommitIds> = Vec::new();
        let mut lane_index_for_commit: HashMap<String, usize> = HashMap::new();
        for branch in state.branches.values() {
            let mut head = state.branch_head(&branch.name);
            let mut lane_commits: Vec<&String> = Vec::new();
            while let Some(commit_id) = head {
                let state_commit = state.commits.get(commit_id).unwrap();
                if state_commit.branch == branch.name {
                    lane_commits.push(commit_id);
                    lane_index_for_commit.insert(commit_id.clone(), commit_id_lanes.len());
                }
                head = state_commit.parents.first();
            }
            commit_id_lanes.push(
               LaneWithCommitIds {
                   branch_names: Vec::from([branch.name.clone()]),
                   commit_ids: lane_commits,
                   priority: branch.priority
               }
            );
        }
        // Find the first and last of lanes commits
        let first_commits = commit_id_lanes.iter().fold(
            HashSet::new(),
            |mut res, l| match l.commit_ids.first() {
                None => res,
                Some(&c) => {res.insert(c); res}
            }
        );
        let last_commits = commit_id_lanes.iter().fold(
            HashSet::new(),
            |mut res, l| match l.commit_ids.last() {
                None => res,
                Some(&c) => {res.insert(c); res}
            }
        );
        // List of commits
        let mut commits: HashMap<String, Rc<Commit>> = HashMap::new();
        for id in time_ord_commits {
            let state_commit = state.commits.get(id).unwrap();
            // Get the parents
            let mut parents: Vec<Parent> = Vec::new();
            for parent in &state_commit.parents {
                parents.push(Parent{
                    commit: commits.get(parent).unwrap().clone(),
                    style: "".to_string(),
                    ends_lane: first_commits.contains(parent),
                    begins_lane: last_commits.contains(id),
                    in_lane: lane_index_for_commit.get(id) == lane_index_for_commit.get(parent),
                });
            }

            commits.insert(id.clone(), Rc::new(Commit {
                id: state_commit.id.clone(),
                hash: state_commit.id.clone(),
                message: "".to_string(),
                time: state_commit.time,
                style: "".to_string(),
                parents: parents,
            }));
        }

        // Make the lanes
        let lanes = commit_id_lanes.iter().map(
            |l| Rc::new(Lane {
                branch_names: l.branch_names.clone(),
                col: l.priority,
                commits: l.commit_ids.iter().map(
                    |&id| commits.get(id).unwrap().clone()
                ).collect::<Vec<Rc<Commit>>>()
            })
        ).collect::<Vec<Rc<Lane>>>();

        // List of branches
        let mut branches = HashMap::new();
        for branch in state.branches.values() {
            branches.insert(
                branch.name.clone(),
                Rc::new(Branch {
                    name: branch.name.clone(),
                    head: branch.current_commit.as_ref().and_then(
                        |id| commits.get(id).cloned(),
                    ),
                    style: "".to_string()
                }
                ));
        }

        // Branch heads
        let mut commits_branch_heads: HashMap<String, Vec<Rc<Branch>>> = HashMap::new();
        for (name, branch) in &branches {
            if let Some(commit) = &branch.head {
                if let Some(heads) = commits_branch_heads.get_mut(&commit.id) {
                    heads.push(branch.clone());
                } else {
                    commits_branch_heads.insert(commit.id.clone(), vec![branch.clone()]);
                }
            }
        }

        View {
            commits,
            branches,
            commits_branch_heads,
            lanes
        }
    }
}
