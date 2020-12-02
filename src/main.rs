mod day1;
#[macro_use]
mod lib;

fn main() {
    /*
    let mut prefix : String = "./inputs/".to_string();
    prefix.push_str("day1p1");
    prefix.push_str(".txt");
    let data = std::fs::read_to_string(prefix).expect("Could not open file");
    let mut parsed_data = day1::parse_day1p1(&data);
    let solution = day1::solve_day1p1(&mut parsed_data);
    println!("The answer is {}", solution.unwrap());
    */
    let data = std::fs::read_to_string("./inputs/day1p1.txt").expect("Could not find file.");
    let mut parsed_data = day1::parse_day1(&data);
    let sol = day1::solve_day1p1(&mut parsed_data);
    assert_eq!(902451, sol.unwrap());
    println!("The solution is {}", sol.unwrap())
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
}
