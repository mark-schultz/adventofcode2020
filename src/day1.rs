// From inspection, input are all 4-digit numbers
pub fn parse(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.parse::<u16>().unwrap()).collect()
}

pub fn solve_p1(input: &[u16]) -> Option<u32> {
    const TARGET: u16 = 2020;
    let mut vals: Box<[u16]> = input.to_vec().into_boxed_slice();
    vals.sort_unstable();
    for val in vals.iter() {
        if let Result::Ok(i) = vals.binary_search(&(TARGET - val)) {
            return Some((*val as u32) * (vals[i] as u32));
        }
    }
    None
}

pub fn solve_p2(input: &[u16]) -> Option<u64> {
    const TARGET: u16 = 2020;
    let mut vals: Box<[u16]> = input.to_vec().into_boxed_slice();
    vals.sort_unstable();
    for val1 in vals.iter() {
        for val2 in vals.iter() {
            if val1 + val2 > TARGET {
                ()
            } else if let Result::Ok(k) = vals.binary_search(&(TARGET - val1 - val2)) {
                return Some((*val1 as u64) * (*val2 as u64) * (vals[k] as u64));
            }
        }
    }
    None
}
