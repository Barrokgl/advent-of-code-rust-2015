use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::cmp::max;

fn build_matrix(x: usize, y: usize) -> Vec<Vec<i32>> {
    (0..x).map(|_| vec![0; y]).collect::<Vec<_>>()
}

fn switch_light(command: &str, n: i32) -> i32 {
    match command {
        s if s.starts_with("turn on") => 1,
        s if s.starts_with("turn off") => 0,
        s if s.starts_with("toggle") => {
            if n == 0 {
                1
            } else {
                0
            }
        }
        _ => n,
    }
}

fn switch_grid(
    matrix: &mut Vec<Vec<i32>>,
    command: &str,
    from: (usize, usize),
    to: (usize, usize),
) {
    (from.0..(to.0 + 1)).for_each(|i| {
        (from.1..(to.1 + 1)).for_each(|j| matrix[i][j] = switch_light(command, matrix[i][j]))
    })
}

fn parse_capture(cap: Option<Match>) -> Option<i32> {
    cap.and_then(|x| x.as_str().parse::<i32>().ok())
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?P<command>turn on|turn off|toggle) (?P<first>\d+),(?P<second>\d+) through (?P<third>\d+),(?P<fourth>\d+)"
    ).unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = build_matrix(1000, 1000);
    let res = input.lines().fold(matrix, |mut m, command| {
        let line = RE
            .captures(command)
            .map(|cap| {
                (
                    cap.name("command").and_then(|x| Some(x.as_str())).unwrap(),
                    parse_capture(cap.name("first")).unwrap(),
                    parse_capture(cap.name("second")).unwrap(),
                    parse_capture(cap.name("third")).unwrap(),
                    parse_capture(cap.name("fourth")).unwrap(),
                )
            })
            .unwrap();
        switch_grid(
            &mut m,
            command,
            (line.1.try_into().unwrap(), line.2.try_into().unwrap()),
            (line.3.try_into().unwrap(), line.4.try_into().unwrap()),
        );
        // let new_light = switch_light(line.0, n)
        m
    });
    Some(res.iter().map(|xs| xs.iter().sum::<i32>()).sum::<i32>() as u32)
}

fn switch_light_advance(command: &str, n: i32) -> i32 {
    match command {
        s if s.starts_with("turn on") => n + 1,
        s if s.starts_with("turn off") => max(n - 1, 0),
        s if s.starts_with("toggle") => n + 2,
        _ => n,
    }
}

fn switch_grid_advance(
    matrix: &mut Vec<Vec<i32>>,
    command: &str,
    from: (usize, usize),
    to: (usize, usize),
) {
    (from.0..(to.0 + 1)).for_each(|i| {
        (from.1..(to.1 + 1))
            .for_each(|j| matrix[i][j] = switch_light_advance(command, matrix[i][j]))
    })
}
pub fn part_two(input: &str) -> Option<u32> {
    let matrix = build_matrix(1000, 1000);
    let res = input.lines().fold(matrix, |mut m, command| {
        let line = RE
            .captures(command)
            .map(|cap| {
                (
                    cap.name("command").and_then(|x| Some(x.as_str())).unwrap(),
                    parse_capture(cap.name("first")).unwrap(),
                    parse_capture(cap.name("second")).unwrap(),
                    parse_capture(cap.name("third")).unwrap(),
                    parse_capture(cap.name("fourth")).unwrap(),
                )
            })
            .unwrap();
        switch_grid_advance(
            &mut m,
            command,
            (line.1.try_into().unwrap(), line.2.try_into().unwrap()),
            (line.3.try_into().unwrap(), line.4.try_into().unwrap()),
        );
        m
    });
    Some(
        res.iter()
            .map(|xs| xs.iter().filter(|n| n.is_positive()).sum::<i32>())
            .sum::<i32>() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(1000000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(2000000));
    }
}
