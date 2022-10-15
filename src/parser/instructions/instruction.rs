use super::errors::*;
use crate::parser::instructions::arguments::ArgList;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct BranchArgs {
    new_root: bool,
}

impl BranchArgs {
    fn default() -> BranchArgs {
        BranchArgs { new_root: false }
    }

    fn with_new_root(&self) -> BranchArgs {
        let mut res = self.clone();
        res.new_root = true;
        res
    }
}

#[derive(Debug)]
pub enum Instruction {
    BRANCH(String, BranchArgs),
    COMMIT(String),
    CHECKOUT(String),
    MERGE(String, Vec<String>),
}

impl Instruction {
    pub fn from_command(command: String, args: ArgList, line_num: usize) -> Result<Instruction> {
        // Handle the command
        match command.as_str() {
            "branch" => {
                if args.plain_args.len() != 1 {
                    bail!(ErrorKind::WorngNumberOfArguemtns(command, line_num));
                }
                let branch_args =
                    args.named_args
                        .iter()
                        .skip(1)
                        .try_fold(BranchArgs::default(), |ba, arg| match arg.name.as_str() {
                            "new_root" => Result::Ok(ba.with_new_root()),
                            _ => bail!(ErrorKind::InvalidArgument(
                                command.clone(),
                                arg.name.clone(),
                                line_num
                            )),
                        })?;
                Ok(Instruction::BRANCH(args.plain_args[0].clone(), branch_args))
            }
            "commit" => {
                if args.plain_args.len() != 1 || args.named_args.len() != 0 {
                    bail!(ErrorKind::WorngNumberOfArguemtns(command, line_num));
                }
                Ok(Instruction::COMMIT(args.plain_args[0].clone()))
            }
            "checkout" => {
                if args.plain_args.len() != 1 || args.named_args.len() != 0 {
                    bail!(ErrorKind::WorngNumberOfArguemtns(command, line_num));
                }
                Ok(Instruction::CHECKOUT(args.plain_args[0].clone()))
            }
            "merge" => {
                if args.plain_args.len() < 2 || args.named_args.len() != 0 {
                    bail!(ErrorKind::WorngNumberOfArguemtns(command, line_num));
                }
                Ok(Instruction::MERGE(
                    args.plain_args[0].clone(),
                    args.plain_args.iter().skip(1).map(|a| a.clone()).collect(),
                ))
            }
            _ => {
                bail!(ErrorKind::InvalidInstruction(command, line_num));
            }
        }
    }

    pub fn from_line(line: &str, line_num: usize) -> Result<Instruction> {
        lazy_static! {
            static ref COMMAND_RE: Regex =
                Regex::new(r"^(\s*[a-z]+)\((?:([^,\n]+),)*([^,\n]+)\)\s*$").unwrap();
        }
        // Parse the instruction
        match COMMAND_RE.captures(line) {
            None => {
                bail!(ErrorKind::InvalidInstruction(line.to_string(), line_num));
            }
            Some(cs) => {
                let command = cs.get(1).unwrap().as_str().to_string();
                let mut args = Vec::new();
                for i in 2..cs.len() {
                    if let Some(c) = cs.get(i) {
                        args.push(c.as_str().to_string());
                    }
                }
                Instruction::from_command(command, ArgList::parse(args, line_num)?, line_num)
            }
        }
    }
}
