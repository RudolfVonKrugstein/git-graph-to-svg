mod errors;
mod instruction;
mod parser;
pub use instruction::*;
mod arguments;

pub use parser::parse_git_instructions;
