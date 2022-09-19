use crate::model::model::Model;
use std::cmp::max;
use std::collections::HashMap;

// Data we remember for every branch
struct BranchData {
    // The commit line
    commit_line: Vec<char>,
    // The commits we that are going to merge into this branch
    // with the next commit
    merge_into_commits: Vec<String>,
}

struct CommitData {
    commit_id: String,
    branch: String,
    merge_into_branches: Vec<String>,
    merge_from_branches: Vec<String>,
}

fn build_branch_data(input: String) -> Result<HashMap<String, BranchData>, String> {
    let mut branch_datas = HashMap::new();

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
        branch_datas.insert(
            name.to_string(),
            BranchData {
                commit_line: commits.chars().collect::<Vec<char>>(),
                merge_into_commits: Vec::new(),
            },
        );
        max_commit_length = max(max_commit_length, commits.len())
    }
    Ok(branch_datas)
}

fn build_commit_data(
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

fn parse_graph(input: String) -> Result<Model, String> {
    let mut result = Model::new();

    let mut branch_datas = build_branch_data(input)?;

    // Add the branches
    let mut max_commit_length = 0;
    for (name, branch_data) in branch_datas.iter() {
        // Insert the branch
        max_commit_length = max(max_commit_length, branch_data.commit_line.len());
        result.add_branch(name.to_string(), "default".to_string());
    }

    // Go through all commits
    for index in 0..max_commit_length {
        if let Some(commit_data) = build_commit_data(&branch_datas, index) {
            // Collect the parents
            {
                let mut parents = Vec::new();
                for b in commit_data.merge_from_branches {
                    match result.get_branch_last_commit(&b) {
                        None => return Err("Cannot merge from empty branch".to_string()),
                        Some(id) => parents.push(id.clone()),
                    }
                }
                match result.get_branch_last_commit(&commit_data.branch) {
                    None => {}
                    Some(c) => {
                        parents.push(c.clone());
                    }
                }
                result.add_commit(
                    commit_data.commit_id.clone(),
                    commit_data.branch,
                    "".to_string(),
                    parents,
                )?;
            }
            // Remember the commit for all branches that want to merge it
            for target_branch in commit_data.merge_into_branches {
                branch_datas
                    .get_mut(&target_branch)
                    .unwrap()
                    .merge_into_commits
                    .push(commit_data.commit_id.clone());
            }
        }
    }
    Ok(result)
}

mod test {
    use crate::parser::parser::parse_graph;

    // Parse a somple graph!
    fn test_simple_graph() {
        // Setup
        let input = "main: A".to_string();

        // Act
        let result = parse_graph(input).unwrap();

        // Test
        assert_eq!(result.commits.len(), 1);
        assert_eq!(result.branches.len(), 1);
        assert_eq!(
            result.get_branch_last_commit(&"main".to_string()).unwrap(),
            &"A".to_string()
        );
    }
}
