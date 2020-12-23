use std::collections::HashMap;
use std::convert::TryFrom;

impl From<&str> for UnfilledUnvalidatedPassport {
    fn from(input: &str) -> UnfilledUnvalidatedPassport {
        let mut parsed_inputs = HashMap::new();
        let input = input.replace("\n", " ");
        let input = input.trim();
        for kv_pair in input.split(" ") {
            let inps: Vec<_> = kv_pair.split(":").collect();
            assert_eq!(inps.len(), 2);
            let k = inps[0];
            let v = inps[1];
            parsed_inputs.insert(k, v);
        }
        let extract = move |key| parsed_inputs.get(key).map(|&s| s.to_string());
        UnfilledUnvalidatedPassport {
            byr: extract("byr"),
            iyr: extract("iyr"),
            eyr: extract("eyr"),
            hgt: extract("hgt"),
            ecl: extract("ecl"),
            hcl: extract("hcl"),
            pid: extract("pid"),
            cid: extract("cid"),
        }
    }
}

#[derive(Debug)]
struct UnfilledUnvalidatedPassport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[derive(Debug)]
pub struct UnvalidatedPassport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    ecl: String,
    hcl: String,
    pid: String,
    cid: Option<String>,
}

#[derive(Debug)]
pub enum LenUnit {
    In(u16),
    Cm(u16),
}

impl TryFrom<&str> for LenUnit {
    type Error = &'static str;
    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        let (val, unit) = inp.split_at(inp.len() - 2);
        let parsed_val = val.parse::<u16>().map_err(|_| "Failed to parse int")?;
        match unit {
            x if x == "in" => Ok(LenUnit::In(parsed_val)),
            x if x == "cm" => Ok(LenUnit::Cm(parsed_val)),
            _ => Err("Failed to parse unit"),
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: LenUnit,
    ecl: String,
    hcl: String,
    pid: usize,
    cid: Option<u16>,
}

impl TryFrom<UnfilledUnvalidatedPassport> for UnvalidatedPassport {
    type Error = &'static str;

    fn try_from(value: UnfilledUnvalidatedPassport) -> Result<Self, Self::Error> {
        Ok(UnvalidatedPassport {
            byr: value.byr.ok_or("Field byr not found")?,
            iyr: value.iyr.ok_or("Field iyr not found")?,
            eyr: value.eyr.ok_or("Field eyr not found")?,
            hgt: value.hgt.ok_or("Field hgt not found")?,
            ecl: value.ecl.ok_or("Field ecl not found")?,
            hcl: value.hcl.ok_or("Field hcl not found")?,
            pid: value.pid.ok_or("Field pid not found")?,
            cid: value.cid,
        })
    }
}

impl TryFrom<UnvalidatedPassport> for Passport {
    type Error = &'static str;

    fn try_from(input: UnvalidatedPassport) -> Result<Self, Self::Error> {
        // Really should make a macro for below parsing
        let byr = {
            if let Ok(byr) = input.byr.parse::<u16>() {
                if 1920 <= byr && byr <= 2002 {
                    Ok(byr)
                } else {
                    Err("byr in invalid range")
                }
            } else {
                Err("Issue parsing byr as int")
            }
        }?;
        let iyr = {
            if let Ok(iyr) = input.iyr.parse::<u16>() {
                if 2010 <= iyr && iyr <= 2020 {
                    Ok(iyr)
                } else {
                    Err("iyr in invalid range")
                }
            } else {
                Err("Issue parsing iyr as int")
            }
        }?;
        let eyr = {
            if let Ok(eyr) = input.eyr.parse::<u16>() {
                if 2020 <= eyr && eyr <= 2030 {
                    Ok(eyr)
                } else {
                    Err("eyr in invalid range")
                }
            } else {
                Err("Issue parsing eyr as int")
            }
        }?;
        let hgt = LenUnit::try_from(&input.hgt)?;
        Err("This is included to make the type checker happy")
    }
}

pub fn parse(input: &str) -> Vec<Result<UnvalidatedPassport, &str>> {
    input
        .split("\n\n")
        .map(|block| UnvalidatedPassport::try_from(UnfilledUnvalidatedPassport::from(block)))
        .collect()
}

pub fn solve_p1(input: &[Result<UnvalidatedPassport, &str>]) -> usize {
    for val in input.iter() {
        dbg!(val);
    }
    input.iter().flatten().count()
}
