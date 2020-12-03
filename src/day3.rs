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

#[derive(Copy, Debug, Clone)]
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
pub fn parse(input: &str) -> Result<Map, Error> {
    let content: Vec<Vec<_>> = input
        .lines()
        .filter(|line| line.len() != 0)
        .map(|line| line.as_bytes().iter().map(|&b| Square::new(b)).collect())
        .collect::<Result<_, Error>>()?;
    Ok(Map {
        width: content[0].len(),
        height: content.len(),
        array: content,
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
pub fn solve_p1(input: &Map) -> usize {
    const PT: Point = Point { row: 1, col: 3 };
    solve_slope(input, PT)
}

#[inline]
pub fn solve_p2(input: &Map) -> usize {
    const POINTS: [Point; 5] = [
        Point { col: 1, row: 1 },
        Point { col: 3, row: 1 },
        Point { col: 5, row: 1 },
        Point { col: 7, row: 1 },
        Point { col: 1, row: 2 },
    ];
    POINTS.iter().map(|pt| solve_slope(input, *pt)).product()
}
