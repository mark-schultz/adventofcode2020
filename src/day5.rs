#[derive(Debug, PartialEq, Clone)]
pub struct BoardingPass {
    row: u32,
    col: u32,
    id: u32,
}

impl Eq for BoardingPass {}

fn interpret_string_as_binary(input: &str, zero_byte: u8, one_byte: u8) -> Option<u32> {
    let mut res: u32 = 0;
    for val in input.as_bytes() {
        res <<= 1;
        res += {
            match *val {
                v if v == zero_byte => Some(0),
                v if v == one_byte => Some(1),
                _ => None,
            }?
        };
    }
    Some(res)
}

fn parse_pass(input: &str) -> Option<BoardingPass> {
    const F: u8 = "F".as_bytes()[0];
    const B: u8 = "B".as_bytes()[0];
    const L: u8 = "L".as_bytes()[0];
    const R: u8 = "R".as_bytes()[0];
    let (vert, horiz) = input.split_at(7);
    let row = interpret_string_as_binary(&vert, F, B)?;
    let col = interpret_string_as_binary(&horiz, L, R)?;
    Some(BoardingPass {
        row,
        col,
        id: 8 * row + col,
    })
}

pub fn parse(input: &str) -> Vec<BoardingPass> {
    input
        .lines()
        .map(|line| parse_pass(&line))
        .filter(|pass| pass.is_some())
        .map(|pass| pass.expect("A `None` pass got through the filter?"))
        .collect()
}

pub fn solve_p1(input: &[BoardingPass]) -> u32 {
    input
        .iter()
        .map(|bp| bp.id)
        .max()
        .expect("Input should be non-empty")
}

pub fn solve_p2(input: &[BoardingPass]) -> Option<u32> {
    let front: u32 = input.iter().map(|bp| bp.row).min()?;
    let back: u32 = input.iter().map(|bp| bp.row).max()?;
    let mut possible_ids: Vec<u32> = input
        .iter()
        .filter(|&bp| bp.row != front && bp.row != back)
        .map(|bp| bp.id)
        .collect();
    let min_id: u32 = (possible_ids.iter().min()?).clone();
    possible_ids.sort();
    let normalized_ids: Vec<u32> = possible_ids.iter_mut().map(|&mut id| id - min_id).collect();
    for (idx, &val) in normalized_ids.iter().enumerate() {
        if (idx as u32) != val {
            return Some(idx as u32 + min_id);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boarding_pass() {
        const INPUT: &str = "BFFFBBFRRR";
        let bp = parse_pass(INPUT).unwrap();
        let expected = BoardingPass {
            row: 70,
            col: 7,
            id: 567,
        };
        assert_eq!(bp, expected)
    }
}
