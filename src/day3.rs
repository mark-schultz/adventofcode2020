use std::iter::Iterator;
use std::ops::{Index, Range};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Square {
    Tree,
    Open,
}

impl Eq for Square {}

#[derive(Debug)]
pub enum Error {
    ParseCharError(String),
    ParseMapError(String),
}

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

#[inline]
pub fn parse_day3p1(input: &str) -> Result<Map, Error> {
    let lines = input.clone().lines();
    let content: Vec<Vec<_>> = lines
        .map(|line| line.as_bytes().iter().map(|&b| Square::new(b)).collect())
        .collect::<Result<_, Error>>()?;
    let nonzero: Vec<Vec<_>> = content.into_iter().filter(|line| line.len() > 0).collect();
    Ok(Map {
        width: nonzero[0].len(),
        height: nonzero.len(),
        array: nonzero,
    })
}

fn solve_slope(input: &Map, slope: Point) -> usize {
    Range {
        start: 1,
        end: input.height / slope.row,
    }
    .map(|i| Point {
        row: i * slope.row,
        col: i * slope.col,
    })
    .filter(|pt| input[*pt] == Square::Tree)
    .count()
}

#[inline]
pub fn solve_day3p1(input: &Map) -> usize {
    const PT: Point = Point { row: 1, col: 3 };
    solve_slope(input, PT)
}

#[inline]
pub fn solve_day3p2(input: &Map) -> usize {
    const POINTS: [Point; 5] = [
        Point { col: 1, row: 1 },
        Point { col: 3, row: 1 },
        Point { col: 5, row: 1 },
        Point { col: 7, row: 1 },
        Point { col: 1, row: 2 },
    ];
    POINTS.iter().map(|pt| solve_slope(input, *pt)).product()
}
