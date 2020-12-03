#[derive(Debug)]
pub struct Password {
    nums: Vec<u32>,
    constrained_char: char,
    password: String,
}
/// Input lines are of the form "X-Y c: abcd..."
fn line_to_password(input: &str) -> Option<Password> {
    let mut line = input.clone().split_whitespace();
    let nums = line
        .next()?
        .split("-")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let constrained_char = line.next()?.chars().next()?;
    let password = line.next()?;
    Some(Password {
        nums: nums,
        constrained_char: constrained_char,
        password: password.to_string(),
    })
}

fn validate_password_range(input: &Password) -> bool {
    assert!(input.nums.len() >= 2);
    let num_range = std::ops::RangeInclusive::new(input.nums[0] as usize, input.nums[1] as usize);
    num_range.contains(
        &input
            .password
            .chars()
            .filter(|c| c == &input.constrained_char)
            .count(),
    )
}

fn validate_password_indices(input: &Password) -> Option<bool> {
    Some(
        (input.password.chars().nth((input.nums[0] - 1) as usize)? == input.constrained_char)
            ^ (input.password.chars().nth((input.nums[1] - 1) as usize)? == input.constrained_char),
    )
}

pub fn parse_day2p1(input: &str) -> Vec<Password> {
    input.lines().filter_map(|l| line_to_password(l)).collect()
}

pub fn solve_day2p1(input: &[Password]) -> usize {
    input
        .into_iter()
        .filter(|p| validate_password_range(p))
        .count()
}

pub fn solve_day2p2(input: &[Password]) -> usize {
    input
        .into_iter()
        .filter(|p| validate_password_indices(p).unwrap_or_default())
        .count()
}
