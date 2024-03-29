use super::errors::*;
use super::instruction::*;
use crate::model::repo::*;

pub fn parse_git_instructions(input: &str) -> Result<Repository> {
    // Go through input line by line
    let lines = input.split("\n");
    let mut state = Repository::default();
    for (line_num, line) in lines.enumerate() {
        let line = line.trim();
        // Ignore empty instructions
        if line.is_empty() {
            continue;
        }

        // Parse the instruction
        let command = Instruction::from_line(line, line_num)?;

        state.apply_instruction(&command);
    }
    Ok(state)
}
