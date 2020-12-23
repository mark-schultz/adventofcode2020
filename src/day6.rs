#![feature(test)]

fn parse_iterator(input: &str) -> Option<Vec<u32>> {
    input
        .lines()
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| c.to_digit(36).map(|d| 1 << (d - 10)))
                .sum()
        })
        .collect()
}

fn parse_looping(input: &str) -> Option<Vec<u32>> {
    let mut vec: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut val = 0;
        for chr in line.trim().chars() {
            // Interpret a char in [a-z] as a value in [0,25]
            // `to_digit` interprets [0-9] as [0-9], so we have
            // to subtract that off
            val += 1 << (chr.to_digit(36)? - 10);
        }
        vec.push(val)
    }
    Some(vec)
}

/// First parse into lines, splitting on 'completely empty' lines
pub fn parse(input: &str) -> Option<Vec<Vec<u32>>> {
    input
        .split("\n\n")
        .map(|line| parse_looping(&line))
        .collect()
}
pub fn parse_iter(input: &str) -> Option<Vec<Vec<u32>>> {
    input
        .split("\n\n")
        .map(|line| parse_iterator(&line))
        .collect()
}

pub fn solve_p1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|vec| vec.iter().fold(0, |acc, elem| acc | elem).count_ones())
        .sum()
}

pub fn solve_p2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|vec| {
            vec.iter()
                .fold(u32::MAX, |acc, elem| acc & elem)
                .count_ones()
        })
        .sum()
}
