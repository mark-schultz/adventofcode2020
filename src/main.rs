#![allow(dead_code)]
#![feature(min_const_generics)]

mod day1;
mod day2;
mod day3;
//mod day4;
mod day5;
mod day6;
#[macro_use]
mod lib;

fn main() {
    let data = std::fs::read_to_string("./inputs/day6.txt").expect("Could not find file.");
    let parsed_data = day6::parse(&data).unwrap();
    let sol = day6::solve_p2(&parsed_data);
    dbg!(sol);
    // 6249
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day1p1() {
        let data = std::fs::read_to_string("./inputs/day1p1.txt").expect("Could not find file.");
        let mut parsed_data = day1::parse(&data);
        let sol = day1::solve_p1(&mut parsed_data);
        assert_eq!(902451, sol.unwrap());
    }
    #[test]
    fn test_day1p2() {
        let data = std::fs::read_to_string("./inputs/day1p1.txt").expect("Could not find file.");
        let mut parsed_data = day1::parse(&data);
        let sol = day1::solve_p2(&mut parsed_data);
        assert_eq!(85555470, sol.unwrap());
    }
    #[test]
    fn test_day2p1() {
        let data = std::fs::read_to_string("./inputs/day2p1.txt").expect("Could not find file.");
        let data = day2::parse(&data);
        let sol = day2::solve_p1(&data);
        assert_eq!(sol, 416);
    }
    #[test]
    fn test_day2p2() {
        let data = std::fs::read_to_string("./inputs/day2p1.txt").expect("Could not find file.");
        let data = day2::parse(&data);
        let sol = day2::solve_p2(&data);
        assert_eq!(688, sol)
    }
    #[test]
    fn test_day3p1() {
        let data = std::fs::read_to_string("./inputs/day3p1.txt").expect("Could not find file.");
        let data = day3::parse(&data).unwrap();
        let sol = day3::solve_p1(&data);
        assert_eq!(257, sol)
    }
    #[test]
    fn test_day3p2() {
        let data = std::fs::read_to_string("./inputs/day3p1.txt").expect("Could not find file.");
        let data = day3::parse(&data).unwrap();
        let sol = day3::solve_p2(&data);
        assert_eq!(1744787392, sol);
    }
    #[test]
    fn test_day5p1() {
        let data = std::fs::read_to_string("./inputs/day5.txt").expect("Could not find file.");
        let parsed_data = day5::parse(&data);
        let sol = day5::solve_p1(&parsed_data);
        assert_eq!(sol, 818);
    }
    #[test]
    fn test_day5p2() {
        let data = std::fs::read_to_string("./inputs/day5.txt").expect("Could not find file.");
        let parsed_data = day5::parse(&data);
        let sol = day5::solve_p2(&parsed_data).unwrap();
        assert_eq!(sol, 559);
    }
    #[test]
    fn test_day6p1() {
        let data = std::fs::read_to_string("./inputs/day6.txt").expect("Could not find file.");
        let parsed_data = day6::parse(&data).unwrap();
        let sol = day6::solve_p1(&parsed_data);
        assert_eq!(6249, sol);
    }
    #[test]
    fn test_day6p2() {
        let data = std::fs::read_to_string("./inputs/day6.txt").expect("Could not find file.");
        let parsed_data = day6::parse(&data).unwrap();
        let sol = day6::solve_p2(&parsed_data);
        assert_eq!(sol, 3103);
    }
}
