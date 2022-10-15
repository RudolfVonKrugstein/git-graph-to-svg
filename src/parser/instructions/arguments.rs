use super::errors::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
pub struct NamedArg {
    pub name: String,
    pub value: String
}

impl NamedArg {
    fn new(name: String, value: String) -> NamedArg {
        NamedArg {name, value}
    }

    pub fn parse(arg: &str) -> Option<NamedArg> {
        lazy_static! {
            static ref NAMED_ARG_RE: Regex = Regex::new(r"^\s*([a-zA-Z]+)\s*=(.+)$").unwrap();
        }
        NAMED_ARG_RE.captures(arg).map(
            |res| NamedArg {
                name: res.get(1).unwrap().as_str().trim().to_string(),
                value: res.get(2).unwrap().as_str().trim().to_string(),
            }
        )
    }
}

#[derive(Clone)]
pub struct ArgList {
    pub plain_args: Vec<String>,
    pub named_args: Vec<NamedArg>
}

impl ArgList {

    fn new() -> ArgList {
        ArgList {
            plain_args: Vec::new(),
            named_args: Vec::new(),
        }
    }

    pub fn parse(args: Vec<String>, line_num: usize) -> Result<ArgList> {
        args.iter().try_fold(
            ArgList::new(),
            |list, arg| {
                let mut res = list.clone();
                match NamedArg::parse(arg) {
                    Some(n) => {
                        res.named_args.push(n)
                    }
                    None => {
                        if list.named_args.len() > 0 {
                            bail!(ErrorKind::NamedArgAfterPlainArg(line_num))
                        }
                        res.plain_args.push(arg.clone())
                    }
                };
                Ok(res)
            }
        )
    }
}
