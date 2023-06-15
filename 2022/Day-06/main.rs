#![feature(array_windows)]

use std::collections::HashSet;

fn part_a(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .array_windows::<4>()
        .enumerate()
        .find(|(_, c)| c.into_iter().collect::<HashSet<_>>().len() == c.len())
        .map(|(i, c)| i + c.len())
        .unwrap()
}

fn part_b(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .array_windows::<14>()
        .enumerate()
        .find(|(_, c)| c.into_iter().collect::<HashSet<_>>().len() == c.len())
        .map(|(i, c)| i + c.len())
        .unwrap()
}

fn main() {
    let input = include_str!("input").trim();

    println!("a: {:?}", part_a(input));
    println!("b: {:?}", part_b(input));
}

#[cfg(test)]
mod tests {
    use crate::{part_a, part_b};

    #[test]
    fn test_part_a() {
        assert_eq!(part_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part_a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_b("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_b("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
