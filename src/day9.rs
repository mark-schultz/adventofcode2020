const WINDOW_SIZE: usize = 25;

pub fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.parse::<isize>())
        .flatten()
        .collect()
}

/// A way to represent value as the sum of two indices of a slice
#[derive(Debug, Copy, Clone)]
struct TwoSumRepr {
    indices: (usize, usize),
    value: isize,
}

/// Returns all pairs of indices s.t. input[i] + input[j] == target
/// Naive quadratic solution because slices will be O(1) size, and because I don't
/// want to have to track the original indices through the sort
fn two_sum(input: &[isize], target: isize) -> Vec<TwoSumRepr> {
    let mut res: Vec<TwoSumRepr> = Vec::with_capacity(input.len());
    for (i, i_val) in input.iter().enumerate() {
        for (j, j_val) in input.iter().enumerate() {
            if i < j && i_val + j_val == target {
                res.push(TwoSumRepr {
                    indices: (i, j),
                    value: target,
                })
            }
        }
    }
    res
}

fn validate(idx: usize, input: &[isize]) -> bool {
    let window = &input[idx - WINDOW_SIZE..idx];
    let two_sum_reprs = two_sum(window, input[idx]);
    two_sum_reprs.len() > 0
}

/// Find the first number in the list which is NOT a two_sum
/// of the prior 25 numbers
pub fn solve_p1(input: &[isize]) -> Option<isize> {
    for idx in WINDOW_SIZE..input.len() {
        if !validate(idx, input) {
            return Some(input[idx]);
        }
    }
    None
}

pub fn solve_p2(input: &[isize]) -> Option<isize> {
    let mut target_idx: usize = 0;
    for idx in WINDOW_SIZE..input.len() {
        if !validate(idx, input) {
            target_idx = idx;
            break;
        }
    }
    for lower_range in 0..target_idx {
        for upper_range in (lower_range + 1)..target_idx {
            let range = &input[lower_range..=upper_range];
            if range.iter().sum::<isize>() == input[target_idx] {
                return Some(range.iter().min()? + range.iter().max()?);
            }
        }
    }
    None
}
