use std::io::{BufWriter, Write};
use crate::model::model::Model;
use crate::options::layout::{LayoutDirection, LayoutOptions};
use super::super::errors::*;

pub fn print_pikchr(model: &Model, options: &LayoutOptions) -> Result<String> {
  let mut buf = BufWriter::new(Vec::new());
  // Start with the direction
  match options.graph_direction {
    LayoutDirection::UP => writeln!(buf, "up").map_err(|e| e.to_string())?,
    LayoutDirection::RIGHT => {}
  }

  // Options
  writeln!(buf, "circlerad = {}cm", options.commit_radius)?;

  // Calc the cols of the branches
  let branch_cols = model.calc_branch_columns();

  // Go through the branches
  for (col, branches) in branch_cols.iter().enumerate() {
    for name in branches {
      let branch = model.branches.get(name).unwrap();
      for (index, commit_id) in branch.commits.iter().enumerate() {
        let commit = model.commits.get(commit_id).unwrap();
        if index == 0 {
          // Absolute position the commit
          writeln!(buf, "circle \"{}\" at ({}cm, {}cm)", commit_id, col * options.branch_dist, commit.time * options.commit_hist_dist);
        } else {
          writeln!(buf, "circle \"{}\"", commit_id);
        }
        if index < branch.commits.len() - 1 {
          // Distance in history between this and the next one
          let hist_diff = &model.commits[&branch.commits[index + 1]].time - &model.commits[commit_id].time;
          writeln!(buf, "arrow <- to {}cm above {} chop", hist_diff * options.commit_hist_dist - options.commit_radius, commit_id);
        }
      }
    }
  }

  // Go through commits and print those lines, that are not "in-branch"
  for (id, commit) in model.commits.iter() {
    let branch = &model.branches[&commit.branch];
    for parent in commit.parents.iter() {
      let parent_commit = &model.commits[parent];
      let parent_branch = &model.branches[&parent_commit.branch];

      if parent_commit.branch == commit.branch {
        continue
      }
      // Same data about us ...
      let child_is_first_in_branch = branch.commits.first() == Some(id);
      let parent_is_last_in_branch = parent_branch.commits.last() == Some(parent);

      // Find a mid point
      let mid_on_child_branch = child_is_first_in_branch && !parent_is_last_in_branch;
      let mid_on_parent_branch = parent_is_last_in_branch && !child_is_first_in_branch;
      match (mid_on_child_branch, mid_on_parent_branch) {
        (true,false) => {
          //arrow from D chop then down 2cm then to A
          writeln!(buf, "arrow from {} chop then to {}cm below {} then to {} chop", id, options.branch_dist, id, parent);
        },
        (false,true) => {
          writeln!(buf, "arrow from {} chop then to {}cm above {} then to {} chop", id, options.branch_dist, parent, parent);
        },
        default => {
          // arrow from D chop then to 2cm above G then to G chop
          writeln!(buf, "arrow from {} to {} chop", id, parent);
        }
      }
    }
    if commit.parents.len() == 1 {
      continue;
    }
  }

  Ok(String::from_utf8(buf.into_inner()?)?)
}
