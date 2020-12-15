fn parse_line(input: &str) -> Option<Vec<u32>> {
    let lines = input.lines();
    let n = lines.size_hint().1.unwrap_or(10);
    let mut output: Vec<u32> = Vec::with_capacity(n);
    for line in lines {
        let mut val: u32 = 0;
        let parsed_line = line.trim().chars();
        for chr in parsed_line {
            // Interpret a char in [a-z] as a value in [0,25]
            // `to_digit` interprets [0-9] as [0-9], so we have
            // to subtract that off
            val += 1 << (chr.to_digit(36)? - 10);
        }
        output.push(val)
    }
    Some(output)
}

/// First parse into lines, splitting on 'completely empty' lines
pub fn parse(input: &str) -> Option<Vec<Vec<u32>>> {
    input.split("\n\n").map(|line| parse_line(&line)).collect()
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
