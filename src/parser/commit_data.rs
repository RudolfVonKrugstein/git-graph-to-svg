use std::collections::HashMap;
use crate::parser::branch_data::BranchData;

pub struct CommitData {
    pub commit_id: String,
    pub branch: String,
    pub merge_into_branches: Vec<String>,
    pub merge_from_branches: Vec<String>,
}



pub fn build_commit_data(
    branch_datas: &Vec<BranchData>,
    commit_index: usize,
) -> Option<CommitData> {
    let mut commit_and_branch: Option<(String, String)> = None;
    let mut merge_into_branches = Vec::new();
    let mut merge_from_branches = Vec::new();

    // Test if there is any commit on any branch ..
    for branch_data in branch_datas.iter() {
        let commit_symbol = branch_data.commit_line.get(commit_index).unwrap_or(&' ');
        if commit_symbol.is_alphanumeric() {
            commit_and_branch = Some((
                branch_data.commit_line[commit_index].to_string(),
                branch_data.name.clone(),
            ));
        }
        if commit_symbol == &'<' {
            merge_into_branches.push(branch_data.name.clone());
        }
        if commit_symbol == &'>' {
            merge_from_branches.push(branch_data.name.clone());
        }
    }
    commit_and_branch.map(|(commit_id, branch)| CommitData {
        commit_id,
        branch,
        merge_into_branches,
        merge_from_branches,
    })
}
