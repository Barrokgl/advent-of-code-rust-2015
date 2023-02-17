pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().fold(0, |acc, symbol| match symbol {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let default: (i32, Option<i32>) = (0, None);
    input
        .chars()
        .enumerate()
        .fold(default, |acc, symbol| match symbol.1 {
            '(' => (acc.0 + 1, acc.1),
            ')' if acc.1.is_none() && acc.0 - 1 == -1 => {
                (acc.0 - 1, Some((symbol.0 + 1).try_into().unwrap()))
            }
            ')' => (acc.0 - 1, acc.1),
            _ => acc,
        })
        .1
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(-3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(1));
    }
}
