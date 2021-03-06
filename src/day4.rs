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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
enum LenUnit {
    In(u16),
    Cm(u16),
}

impl TryFrom<&str> for LenUnit {
    type Error = &'static str;
    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        let (val, unit) = inp.split_at(inp.len() - 2);
        let parsed_val = val.parse::<u16>().map_err(|_| "Failed to parse int")?;
        match unit {
            "in" => Ok(LenUnit::In(parsed_val)),
            "cm" => Ok(LenUnit::Cm(parsed_val)),
            _ => Err("Failed to parse unit"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EyeColor {
    AMB,
    BLU,
    BRN,
    GRY,
    GRN,
    HZL,
    OTH,
}

impl TryFrom<&str> for EyeColor {
    type Error = &'static str;
    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        match inp {
            "amb" => Ok(EyeColor::AMB),
            "blu" => Ok(EyeColor::BLU),
            "brn" => Ok(EyeColor::BRN),
            "gry" => Ok(EyeColor::GRY),
            "grn" => Ok(EyeColor::GRN),
            "hzl" => Ok(EyeColor::HZL),
            "oth" => Ok(EyeColor::OTH),
            _ => Err("Cannot parse eye color"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: LenUnit,
    ecl: EyeColor,
    hcl: String,
    pid: usize,
    cid: Option<String>,
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
        let hgt = LenUnit::try_from(input.hgt.as_str())?;
        let _ = {
            match hgt {
                LenUnit::Cm(s) if 150 <= s && s <= 193 => Ok(s),
                LenUnit::In(s) if 59 <= s && s <= 76 => Ok(s),
                _ => Err("Height in invalid range."),
            }?
        };
        let hcl = {
            if input.hcl.chars().next() == Some('#') {
                if input.hcl[1..]
                    .chars()
                    .all(|s| s.is_ascii_digit() || "abcdef".contains(s))
                {
                    Ok(input.hcl)
                } else {
                    Err("Hex string contains non-0..9..a..f chars")
                }
            } else {
                Err("Hex string not delimited by #")
            }
        }?;
        let ecl = EyeColor::try_from(input.ecl.as_str())?;
        let _ = if input.pid.len() != 9 {
            Err("PID is wrong length")
        } else {
            Ok(true)
        }?;
        let pid = input
            .pid
            .parse::<usize>()
            .map_err(|_| "Failed to parse PID")?;
        let cid = input.cid;
        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            ecl,
            hcl,
            pid,
            cid,
        })
    }
}

pub fn parse(input: &str) -> Vec<Result<UnvalidatedPassport, &str>> {
    input
        .split("\n\n")
        .map(|block| UnvalidatedPassport::try_from(UnfilledUnvalidatedPassport::from(block)))
        .collect()
}

pub fn solve_p1(input: &[Result<UnvalidatedPassport, &str>]) -> usize {
    input.iter().flatten().count()
}

pub fn solve_p2(input: &[Result<UnvalidatedPassport, &str>]) -> usize {
    input
        .iter()
        .flatten()
        .map(|p| Passport::try_from(p.clone()))
        .flatten()
        .count()
}
