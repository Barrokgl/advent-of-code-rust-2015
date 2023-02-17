use std::collections::HashMap;
// x, y
#[derive(Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    pub fn move_point(&self, direction: char) -> Point {
        match direction {
            '^' => Point(self.0 + 1, self.1),
            'v' => Point(self.0 - 1, self.1),
            '>' => Point(self.0, self.1 + 1),
            '<' => Point(self.0, self.1 - 1),
            _ => Point(self.0, self.1),
        }
    }
    pub fn to_tuple(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let start_point = Point(0, 0);
    map.insert(start_point.to_tuple(), 1);
    let default: (Point, HashMap<(i32, i32), i32>) = (start_point, map);
    let result: usize = input
        .chars()
        .fold(default, |mut acc, direction| {
            let new_point = acc.0.move_point(direction);
            *acc.1.entry(new_point.to_tuple()).or_insert(0) += 1;
            (new_point, acc.1)
        })
        .1
        .iter()
        .len();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let start_point_santa = Point(0, 0);
    let start_point_robot = Point(0, 0);
    map.insert(start_point_santa.to_tuple(), 1);
    let default: (Point, Point, HashMap<(i32, i32), i32>) =
        (start_point_santa, start_point_robot, map);
    let result: usize = input
        .chars()
        .enumerate()
        .fold(default, |mut acc, direction| {
            let is_santa_turn = direction.0 % 2 == 0;

            if is_santa_turn {
                let new_point = acc.0.move_point(direction.1);
                *acc.2.entry(new_point.to_tuple()).or_insert(0) += 1;
                (new_point, acc.1, acc.2)
            } else {
                let new_point = acc.1.move_point(direction.1);
                *acc.2.entry(new_point.to_tuple()).or_insert(0) += 1;
                (acc.0, new_point, acc.2)
            }
        })
        .2
        .iter()
        .len();
    Some(result as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
