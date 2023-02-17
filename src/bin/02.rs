use std::cmp::min;
// l, w, h
struct Box(u32, u32, u32);

fn parse_num(list: &[&str], index: usize) -> u32 {
    list.get(index).and_then(|s| s.parse::<u32>().ok()).unwrap()
}

impl Box {
    pub fn from_str(s: &str) -> Box {
        let symbols = s.split("x").collect::<Vec<_>>();
        Box(
            parse_num(&symbols, 0),
            parse_num(&symbols, 1),
            parse_num(&symbols, 2),
        )
    }

    // surface of the box 2*l*w + 2*w*h + 2*h*l
    pub fn surface_area(&self) -> u32 {
        (2 * self.0 * self.1) + (2 * self.1 * self.2) + (2 * self.2 * self.0)
    }

    pub fn min(&self) -> u32 {
        min(self.0 * self.1, min(self.1 * self.2, self.2 * self.0))
    }

    pub fn ribbon(&self) -> u32 {
        let mut dimensions = vec![self.0, self.1, self.2];
        dimensions.sort();

        (self.0 * self.1 * self.2)
            + dimensions.get(0).map(|n| n + n).unwrap()
            + dimensions.get(1).map(|n| n + n).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|box_str| {
                let box_el = Box::from_str(box_str);
                box_el.surface_area() + box_el.min()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|box_str| Box::from_str(box_str).ribbon())
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(58 + 43));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(34 + 14));
    }
}
