#[macro_use]
extern crate error_chain;

pub mod model;
pub mod options;
mod parser;
mod printer;
pub mod view;

pub use parser::instructions::parse_git_instructions;
pub use printer::print_pikchr;

