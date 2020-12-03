use std::ops::{Index, Range};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Square {
    Tree,
    Open,
}

#[derive(Debug)]
pub enum Error {
    ParseCharError(String),
    ParseMapError(String),
}

impl Eq for Square {}

impl Square {
    fn new(c: u8) -> Result<Square, Error> {
        const PERIOD: u8 = ".".as_bytes()[0];
        const HASH: u8 = "#".as_bytes()[0];
        match c {
            PERIOD => Ok(Square::Open),
            HASH => Ok(Square::Tree),
            _ => {
                println!("Encountered byte {} while parsing.", c);
                Err(Error::ParseCharError(String::from(format!(
                    "Error parsing byte {}.",
                    c
                ))))
            }
        }
    }
}

pub struct Map {
    width: usize,
    height: usize,
    array: Vec<Vec<Square>>,
}

#[derive(Copy, Clone)]
struct Point {
    row: usize,
    col: usize,
}

impl Index<Point> for Map {
    type Output = Square;
    fn index(&self, point: Point) -> &Self::Output {
        &self.array[point.row][point.col % self.width]
    }
}

pub fn parse_day3p1(input: &str) -> Result<Map, Error> {
    let lines = input.clone().lines();
    let content: Result<Vec<Vec<Square>>, Error> = lines
        .map(|line| line.as_bytes().iter().map(|&b| Square::new(b)).collect())
        .collect();
    if let Ok(array) = content {
        let nonzero: Vec<Vec<Square>> = array.into_iter().filter(|line| line.len() > 0).collect();
        Ok(Map {
            width: nonzero[0].len(),
            height: nonzero.len(),
            array: nonzero,
        })
    } else {
        Err(Error::ParseMapError(format!("Issue parsing the map.")))
    }
}

pub fn solve_day3p1(input: &Map) -> usize {
    const X_DIFF: usize = 3;
    const Y_DIFF: usize = 1;
    Range {
        start: 1,
        end: input.height,
    }
    .map(|i| Point {
        row: i * Y_DIFF,
        col: i * X_DIFF,
    })
    .filter(|pt| input[*pt] == Square::Tree)
    .count()
}
