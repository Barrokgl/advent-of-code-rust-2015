use md5::{Digest, Md5};

fn get_md5(s: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn part_one(input: &str) -> Option<u32> {
    let xs = vec![input];
    let input_iterator = xs.iter().cycle().enumerate();
    let res = input_iterator
        .take_while(|el| {
            // println!("round: {}", el.0);
            let hash = get_md5(format!("{}{}", el.1, el.0));
            !hash.starts_with("00000")
        })
        .collect::<Vec<_>>();
    Some(res.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let xs = vec![input];
    let input_iterator = xs.iter().cycle().enumerate();
    let res = input_iterator
        .take_while(|el| {
            let hash = get_md5(format!("{}{}", el.1, el.0));
            !hash.starts_with("000000")
        })
        .collect::<Vec<_>>();
    Some(res.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(609043));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(6742839));
    }
}
