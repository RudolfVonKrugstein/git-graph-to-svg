#[macro_use]
extern crate error_chain;

pub mod model;
pub mod options;
mod parser;
mod printer;
pub mod view;

pub use parser::instructions::parse_git_instructions;
pub use printer::print_pikchr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
