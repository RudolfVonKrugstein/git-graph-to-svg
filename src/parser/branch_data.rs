use std::cmp::max;

// Data we remember for every branch
pub struct BranchData {
    // The branch name
    pub name: String,
    // The commit line
    pub(crate) commit_line: Vec<char>,
    // The commits we that are going to merge into this branch
    // with the next commit
    pub(crate) merge_into_commits: Vec<String>,
}

pub fn build_branch_data(input: String) -> Result<Vec<BranchData>, String> {
    let mut branch_datas = Vec::new();

    // Split the input into branch-lines
    let branch_lines = input.split("\n");
    let mut max_commit_length = 0;

    for branch_line in branch_lines {
        let tmp = branch_line.split(":").collect::<Vec<&str>>();
        if tmp.len() != 2 {
            return Err(format!("Incorrect number of : in line {branch_line}"));
        }
        let name = tmp[0].trim();
        let commits = tmp[1];

        // And remember branches data
        branch_datas.push(BranchData {
            name: name.to_string(),
            commit_line: commits.chars().collect::<Vec<char>>(),
            merge_into_commits: Vec::new(),
        });
        max_commit_length = max(max_commit_length, commits.len())
    }
    Ok(branch_datas)
}
