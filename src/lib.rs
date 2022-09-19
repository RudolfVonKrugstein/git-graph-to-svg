#[macro_use]
extern crate error_chain;

mod model;
pub mod options;
mod parser;
mod printer;

pub use parser::parser::parse_graph;
pub use printer::print_pikchr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
