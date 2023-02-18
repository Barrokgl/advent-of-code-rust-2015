use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

fn is_nice(s: &str) -> bool {
    lazy_static! {
        static ref VOWELS: Regex = Regex::new(r"[aeiou]").unwrap();
        static ref BANNED_PAIRS: Regex = Regex::new(r"ab|cd|pq|xy").unwrap();
    }

    let vowels_count = VOWELS.find_iter(s).collect::<Vec<_>>().len();
    let is_banned_pairs = BANNED_PAIRS.is_match(s);
    let doubles_count = s
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|xs| xs.get(0) == xs.get(1))
        .collect::<Vec<_>>()
        .len();
    let res = match (is_banned_pairs, vowels_count, doubles_count) {
        (true, _, _) => false,
        (false, v, d) => v > 2 && d > 0,
    };

    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .filter(|line| is_nice(line))
        .collect::<Vec<_>>();
    Some(res.len() as u32)
}

fn is_really_nice(s: &str) -> bool {
    let map: HashMap<String, HashSet<usize>> = HashMap::new();

    let doubles_count = s
        .chars()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(2)
        .fold(map, |mut acc, el| {
            let st = format!(
                "{}{}",
                el.get(0).map(|item| item.1).unwrap_or('!'),
                el.get(1).map(|item| item.1).unwrap_or('!')
            );
            let set = acc.entry(st).or_insert(HashSet::new());
            match el.get(0) {
                Some(item) => set.insert(item.0),
                _ => false,
            };
            match el.get(1) {
                Some(item) => set.insert(item.0),
                _ => false,
            };
            acc
        })
        .iter()
        .filter(|el| el.1.len() % 2 == 0 && el.1.len() >= 4)
        .collect::<Vec<_>>()
        .len();
    let triple_count = s
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .filter(|xs| xs.get(0) == xs.get(2))
        .collect::<Vec<_>>()
        .len();

    doubles_count > 0 && triple_count > 0
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .filter(|line| is_really_nice(line))
        .collect::<Vec<_>>();
    Some(res.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(2));
    }
}
