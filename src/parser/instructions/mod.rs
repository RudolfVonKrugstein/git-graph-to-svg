mod parser;
mod errors;
mod instruction;
pub use instruction::*;
mod arguments;

pub use parser::parse_git_instructions;
