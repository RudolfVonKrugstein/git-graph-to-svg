use crate::model::model::Model;
use std::cmp::max;
use std::collections::HashMap;
use crate::parser::branch_data::build_branch_data;
use crate::parser::commit_data::build_commit_data;

pub fn parse_graph(input: String) -> Result<Model, String> {
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
