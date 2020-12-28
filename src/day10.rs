use std::convert::TryFrom;
use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Jolt {
    joltage: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Distribution {
    one: usize,
    two: usize,
    three: usize,
}

impl Add<Distribution> for Distribution {
    type Output = Distribution;
    fn add(self, other: Distribution) -> Self::Output {
        Distribution {
            one: self.one + other.one,
            two: self.two + other.two,
            three: self.three + other.three,
        }
    }
}

impl AddAssign<Distribution> for Distribution {
    fn add_assign(&mut self, other: Distribution) {
        self.one += other.one;
        self.two += other.two;
        self.three += other.three;
    }
}

impl TryFrom<usize> for Distribution {
    type Error = &'static str;
    fn try_from(input: usize) -> Result<Self, Self::Error> {
        match input {
            1 => Ok(Distribution {
                one: 1,
                two: 0,
                three: 0,
            }),
            2 => Ok(Distribution {
                one: 0,
                two: 1,
                three: 0,
            }),
            3 => Ok(Distribution {
                one: 0,
                two: 0,
                three: 1,
            }),
            _ => Err("Joltage difference not in {1, 2, 3}"),
        }
    }
}

impl TryFrom<&str> for Jolt {
    type Error = &'static str;
    fn try_from(input: &str) -> Result<Jolt, Self::Error> {
        Ok(Jolt {
            joltage: input.parse::<usize>().map_err(|_| "Failed to parse Jolt")?,
        })
    }
}

pub fn parse(input: &str) -> Vec<Jolt> {
    input.lines().map(|s| Jolt::try_from(s)).flatten().collect()
}

pub fn solve_p1(input: &[Jolt]) -> Result<usize, &'static str> {
    let mut input: Vec<Jolt> = input.clone().into();
    let max_val = input.iter().max().expect("Input is non-empty").joltage;
    input.push(Jolt { joltage: 0 });
    input.push(Jolt {
        joltage: max_val + 3,
    });

    let mut output = Distribution::default();
    input.sort();
    for (idx, connector) in input.iter().enumerate().skip(1) {
        let prior_idx = idx - 1; // fine due to the .skip(1)
        output += Distribution::try_from(connector.joltage - input[prior_idx].joltage)?;
    }
    dbg!(output);
    Ok(output.one * output.three)
}

/// I will assume that only joltage differences of 1 and 3 occur.
/// This probably isn't necessary, but it is implied by my prior solution to the problem.
/// The joltage differences of 3 are "required", so should be able to removed from the problem
/// instance to simplify things.
pub fn solve_p2(input: &[Jolt]) -> usize {
    let mut input: Vec<Jolt> = input.clone().into();
    let max_val = input.iter().max().expect("Input is non-empty").joltage;
    input.push(Jolt { joltage: 0 });
    input.push(Jolt {
        joltage: max_val + 3,
    });
    input.sort();

    todo!();
    0
}
