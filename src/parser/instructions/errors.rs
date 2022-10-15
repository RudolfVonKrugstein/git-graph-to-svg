use std::io::BufWriter;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {

    }

    errors {
        InvalidInstruction(line: String, line_num: usize) {
            description("invalid instruction"),
            display("invalid instruction on line {}: {}", line_num, line),
        }
        WorngNumberOfArguemtns(line: String, line_num: usize) {
            description("wrong number of arguments"),
            display("wrong number of arguments on line {}: {}", line_num, line),
        }
        InvalidArgument(command: String, arg: String, line_num: usize) {
            description("invalid argument"),
            display("invalid argument to {} on line {}: {}", command, line_num, arg),
        }
        NamedArgAfterPlainArg(line_num: usize) {
            description("plain arg after named arg"),
            display("plain arg after named arg on line {}", line_num),
        }
    }
}
