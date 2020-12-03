#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
#[macro_use]
mod lib;

fn main() {
    let data = std::fs::read_to_string("./inputs/day3p1.txt").expect("Could not find file.");
    let data = day3::parse_day3p1(&data).unwrap();
    let sol = day3::solve_day3p2(&data);
    assert_eq!(1744787392, sol);
    println!("The solution is: {}", sol)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day1p1() {
        let data = std::fs::read_to_string("./inputs/day1p1.txt").expect("Could not find file.");
        let mut parsed_data = day1::parse_day1(&data);
        let sol = day1::solve_day1p1(&mut parsed_data);
        assert_eq!(902451, sol.unwrap());
    }
    #[test]
    fn test_day1p2() {
        let data = std::fs::read_to_string("./inputs/day1p1.txt").expect("Could not find file.");
        let mut parsed_data = day1::parse_day1(&data);
        let sol = day1::solve_day1p2(&mut parsed_data);
        assert_eq!(85555470, sol.unwrap());
    }
    #[test]
    fn test_day2p1() {
        let data = std::fs::read_to_string("./inputs/day2p1.txt").expect("Could not find file.");
        let data = day2::parse_day2p1(&data);
        let sol = day2::solve_day2p1(&data);
        assert_eq!(sol, 416);
    }
    #[test]
    fn test_day2p2() {
        let data = std::fs::read_to_string("./inputs/day2p1.txt").expect("Could not find file.");
        let data = day2::parse_day2p1(&data);
        let sol = day2::solve_day2p2(&data);
        assert_eq!(688, sol)
    }
    #[test]
    fn test_day3p1() {
        let data = std::fs::read_to_string("./inputs/day3p1.txt").expect("Could not find file.");
        let data = day3::parse_day3p1(&data).unwrap();
        let sol = day3::solve_day3p1(&data);
        assert_eq!(257, sol)
    }
    #[test]
    fn test_day3p2() {
        let data = std::fs::read_to_string("./inputs/day3p1.txt").expect("Could not find file.");
        let data = day3::parse_day3p1(&data).unwrap();
        let sol = day3::solve_day3p2(&data);
        assert_eq!(1744787392, sol);
    }
}
