use serde::{Deserialize, Serialize};
use serde_json;
use itertools::Itertools;

#[derive(Serialize, Deserialize, Debug)]
pub struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    ecl: String,
    pid: usize,
    cid: Option<u16>,
}

pub fn parse_input_to_json(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|unparsed_pass| unparsed_pass.replace("\n", " ").split(" ").map(|word| word.split(":").skip(1).map(|s| format!(""))
        .collect()
}

pub fn parse(input: &str) -> Vec<Result<Passport, serde_json::Error>> {
    input
        .split("\n\n")
        .map(|line| format!("{{{}}}", line.replace(" ", ",").replace("\n", ",")))
        .map(|s| serde_json::from_str(&s))
        .collect()
}
