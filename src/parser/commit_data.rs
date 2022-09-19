use std::collections::HashMap;
use crate::parser::branch_data::BranchData;

pub struct CommitData {
    pub(crate) commit_id: String,
    pub(crate) branch: String,
    pub(crate) merge_into_branches: Vec<String>,
    pub(crate) merge_from_branches: Vec<String>,
}



pub fn build_commit_data(
    branch_datas: &HashMap<String, BranchData>,
    commit_index: usize,
) -> Option<CommitData> {
    let mut commit_and_branch: Option<(String, String)> = None;
    let mut merge_into_branches = Vec::new();
    let mut merge_from_branches = Vec::new();

    // Test if there is any commit on any branch ..
    for (name, branch_data) in branch_datas.iter() {
        let commit_symbol = branch_data.commit_line.get(commit_index).unwrap_or(&' ');
        if commit_symbol.is_alphanumeric() {
            commit_and_branch = Some((
                branch_data.commit_line[commit_index].to_string(),
                name.clone(),
            ));
        }
        if commit_symbol == &'<' {
            merge_into_branches.push(name.clone());
        }
        if commit_symbol == &'>' {
            merge_from_branches.push(name.clone());
        }
    }
    commit_and_branch.map(|(commit_id, branch)| CommitData {
        commit_id,
        branch,
        merge_into_branches,
        merge_from_branches,
    })
}
