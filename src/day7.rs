//! Part 1 Algorithm idea:
//!     Parsing:
//!         Store all lines a -> [b_1,...,b_k] as a HashMap
//!             (a weighted graph seems *much* better, as I anticipate the numbers in the input
//!             will be relevant for part 2, but I want to stay in `std` if possible)
//!     Solving:
//!         Maintain a HashSet
//!         Iterate through values of hashmap, find ones with the `gold bag` in them and
//!             add to a queue
//!         Pop element off queue, add it to HashSet, and add everything that points to it to the
//!         queue
//!             
//!     This feels like a poor-man's version of graph search (specifically BFS because of the usage
//!     of a queue), but stays in `std`.
//!         

use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub struct ColoredBags<'a> {
    bag_type: ColoredBag<'a>,
    quantity: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub struct ColoredBag<'a> {
    quality: &'a str,
    color: &'a str,
}

impl<'a> From<ColoredBags<'a>> for ColoredBag<'a> {
    fn from(input: ColoredBags<'a>) -> Self {
        input.bag_type
    }
}

impl<'a, 'b: 'a> From<&'b ColoredBags<'a>> for &'b ColoredBag<'a> {
    fn from(input: &'b ColoredBags<'a>) -> Self {
        &(input.bag_type)
    }
}

impl<'a> Eq for ColoredBags<'a> {}
impl<'a> Eq for ColoredBag<'a> {}

impl<'a> TryFrom<&'a str> for ColoredBags<'a> {
    type Error = &'static str;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        const ERR: &'static str = "Issue parsing bag";
        let mut words = input.split(" ");
        let quantity = {
            if words.clone().count() == 3 {
                1
            } else {
                words.next().ok_or(ERR)?.parse::<usize>().map_err(|_| ERR)?
            }
        };
        let quality = words.next().ok_or(ERR)?;
        let color = words.next().ok_or(ERR)?;
        if quality == "no" || color == "other" {
            Err("'Bag' is no other bags")
        } else {
            let bag_type = ColoredBag { quality, color };
            Ok(ColoredBags { bag_type, quantity })
        }
    }
}

fn parse_line(input: &str) -> Option<(ColoredBag, HashSet<ColoredBags>)> {
    let mut iter = input.split("contain");
    let source_bag = iter.next()?.trim();
    let source_bag: ColoredBag = ColoredBags::try_from(source_bag).ok()?.into();
    let target_bag: HashSet<ColoredBags> = iter
        .next()?
        .split(",")
        .map(|s| ColoredBags::try_from(s.trim()).ok())
        .collect::<Option<HashSet<ColoredBags>>>()?;
    Some((source_bag, target_bag))
}

pub fn parse(input: &str) -> HashMap<ColoredBag, HashSet<ColoredBags>> {
    input
        .lines()
        .map(|line| parse_line(line))
        .flatten()
        .collect::<HashMap<ColoredBag, HashSet<ColoredBags>>>()
}

pub fn solve_p1<'a>(
    input: HashMap<ColoredBag<'a>, HashSet<ColoredBags>>,
) -> HashSet<ColoredBag<'a>> {
    let input: HashMap<ColoredBag, HashSet<ColoredBag>> = input
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(|m| m.into()).collect()))
        .collect();
    let mut seen_bags = HashSet::<&ColoredBag>::new();
    let mut queue = VecDeque::new();
    let gold_bag = ColoredBags::try_from("shiny gold bag")
        .expect("At least this needs to parse")
        .into();
    queue.push_back(&gold_bag);
    while !queue.is_empty() {
        let bag = queue.pop_front().expect("Queue is non-empty");
        // Find keys that have bag in values
        for (key, val_set) in &input {
            if val_set.contains(bag) && !seen_bags.contains(key) {
                queue.push_back(key);
                seen_bags.insert(key);
            }
        }
    }
    seen_bags
        .iter()
        .cloned()
        .map(|&x| x)
        .collect::<HashSet<ColoredBag>>()
}

fn recurse_p2<'a>(
    bag: &ColoredBag<'a>,
    bag_map: &HashMap<ColoredBag<'a>, HashSet<ColoredBags>>,
) -> usize {
    if let Some(set) = bag_map.get(bag) {
        let val: usize = set
            .iter()
            .map(|bags| bags.quantity * recurse_p2(bags.into(), &bag_map))
            .sum();
        1 + val
    } else {
        1
    }
}

pub fn solve_p2<'a>(input: HashMap<ColoredBag<'a>, HashSet<ColoredBags>>) -> usize {
    let bag: ColoredBag = ColoredBags::try_from("shiny gold bag").unwrap().into();
    // -1 to remove shiny gold bag from the count
    recurse_p2(&bag, &input) - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_shiny_bag() {
        let bag: ColoredBag = ColoredBags::try_from("shiny gold bag").unwrap().into();
        assert_eq!(
            bag,
            ColoredBag {
                quality: "shiny",
                color: "gold"
            }
        );
    }
}
