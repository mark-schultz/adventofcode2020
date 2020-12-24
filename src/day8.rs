use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    NoOp(isize),
    Acc(isize),
    Jmp(isize),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Termination {
    OutOfBounds,
    InfiniteLoop,
    EndOfProgram,
}

impl Eq for Termination {}

#[derive(Debug, Copy, Clone)]
pub struct ExitReport {
    op_code: Termination,
    acc_value: isize,
}

impl Eq for Op {}

impl TryFrom<&str> for Op {
    type Error = &'static str;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut tokens = input.split(" ");
        let instruction = tokens.next().ok_or("No instruction found")?;
        let value = tokens.next().ok_or("No instruction value found")?;
        let value = value
            .parse::<isize>()
            .map_err(|_| "Error parsing instruction value")?;
        match instruction {
            "nop" => Ok(Op::NoOp(value)),
            "jmp" => Ok(Op::Jmp(value)),
            "acc" => Ok(Op::Acc(value)),
            _ => Err("Error parsing instruction"),
        }
    }
}

pub fn parse(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| Op::try_from(line))
        .flatten()
        .collect()
}

fn execute_program(input: &[Op]) -> ExitReport {
    let mut visited = HashSet::<isize>::new();
    let mut acc: isize = 0;
    let mut idx: isize = 0;
    loop {
        if visited.contains(&idx) {
            return ExitReport {
                op_code: Termination::InfiniteLoop,
                acc_value: acc,
            };
        } else if idx < 0 || idx as usize > input.len() {
            return ExitReport {
                op_code: Termination::OutOfBounds,
                acc_value: acc,
            };
        } else if idx as usize == input.len() {
            return ExitReport {
                op_code: Termination::EndOfProgram,
                acc_value: acc,
            };
        } else {
            visited.insert(idx);
            match input[idx as usize] {
                Op::NoOp(_) => idx += 1,
                Op::Jmp(v) => idx += v,
                Op::Acc(v) => {
                    acc += v;
                    idx += 1
                }
            }
        }
    }
}

pub fn solve_p1(input: &[Op]) -> Result<isize, &'static str> {
    let exit_report = execute_program(input);
    match exit_report.op_code {
        Termination::InfiniteLoop => Ok(exit_report.acc_value),
        Termination::OutOfBounds => Err("Out of bounds program access"),
        Termination::EndOfProgram => Err("Program exited successfully"),
    }
}

fn twiddle_op(idx: usize, program: &[Op]) -> Vec<Op> {
    let mut twiddled_program: Vec<Op> = program.clone().into();
    twiddled_program
        .iter_mut()
        .enumerate()
        .map(|(i, op)| {
            if i == idx {
                match &op {
                    Op::NoOp(v) => Op::Jmp(*v),
                    Op::Jmp(v) => Op::NoOp(*v),
                    Op::Acc(v) => Op::Acc(*v),
                }
            } else {
                *op
            }
        })
        .collect::<Vec<Op>>()
}

pub fn solve_p2(input: &[Op]) -> Option<isize> {
    for idx in 0..input.len() {
        let twiddled_program = twiddle_op(idx, input);
        let exit_report = execute_program(&twiddled_program);
        if Termination::EndOfProgram == exit_report.op_code {
            return Some(exit_report.acc_value);
        }
    }
    None
}
