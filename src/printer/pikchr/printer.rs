use super::super::errors::*;
use crate::model::view::View;
use crate::options::layout::{LayoutDirection, LayoutOptions};
use std::io::{BufWriter, Write};

pub fn print_pikchr(view: &View, options: &LayoutOptions) -> Result<String> {
    // Output buffer
    let mut buf = BufWriter::new(Vec::new());
    // Start with the direction
    match options.graph_direction {
        LayoutDirection::UP => writeln!(buf, "down").map_err(|e| e.to_string())?,
        LayoutDirection::RIGHT => {}
    }

    // Options
    writeln!(buf, "circlerad = {}cm", options.commit_radius)?;

    // Go through the lanes and paint them
    for lane in &view.lanes {
        writeln!(buf, "// branch: {}", lane.branch_names.join(", "))?;
        // First one
        if let Some(first_commit) = lane.commits.first() {
            // absolute position the commit
            writeln!(
                buf,
                "circle \"{}\" at ({}cm, {}cm)",
                first_commit.id,
                lane.col * options.branch_dist,
                first_commit.time * options.commit_hist_dist
            )?;
            let mut last_commit = first_commit;
            // Go through the remaining commits
            for commit in lane.commits.iter().skip(1) {
                let hist_diff = last_commit.time - commit.time;
                // Draw the arrow
                writeln!(
                    buf,
                    "arrow {}cm chop",
                    hist_diff * options.commit_hist_dist - 2 * options.commit_radius
                )?;
                // Draw the new commit
                writeln!(buf, "circle \"{}\"", commit.id)?;
                last_commit = commit;
            }
        }
    }

    // Go through commits and print those lines, that are not "in-lane"
    writeln!(buf, "// out of branch parents")?;
    for (commit_id, commit) in &view.commits {
        for parent in commit.parents.iter() {
            if parent.in_lane {
                continue;
            }
            match (parent.begins_lane, parent.ends_lane) {
                (true, false) => {
                    let down_dist = (commit.time - parent.commit.time) * (options.commit_hist_dist)
                        - (options.branch_dist);
                    writeln!(
                        buf,
                        "arrow from {} chop then to {}cm below {} then to {} chop",
                        commit_id, down_dist, commit_id, parent.commit.id
                    )?;
                }
                (false, true) => {
                    let up_dist = (commit.time - parent.commit.time) * (options.commit_hist_dist)
                        - (options.branch_dist);
                    writeln!(
                        buf,
                        "arrow from {} chop then to {}cm above {} then to {} chop",
                        commit_id, up_dist, parent.commit.id, parent.commit.id
                    )?;
                }
                _ => {
                    writeln!(buf, "arrow from {} to {} chop", commit_id, parent.commit.id)?;
                }
            }
        }
    }

    // Prepare branch tips
    writeln!(buf, "// branch heads")?;
    writeln!(buf, "boxht = 0;")?;

    // Branch tips!
    for (commit, branches) in &view.commits_branch_heads {
        if let Some(branch) = branches.first() {
            // First branch
            writeln!(buf, "right")?;
            writeln!(
                buf,
                "line from {} to ({}cm, {}.y) chop",
                commit,
                view.lanes.len() * options.branch_dist,
                commit
            )?;
            writeln!(buf, "box \"{}\"", branch.name)?;
        }
        for branch in branches.iter().skip(1) {
            // Other branches
            writeln!(buf, "box \"{}\"", branch.name)?;
        }
    }

    Ok(String::from_utf8(buf.into_inner()?)?)
}
