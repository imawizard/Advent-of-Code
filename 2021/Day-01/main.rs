#![feature(array_windows)]

use std::iter::Sum;

fn part_a(input: &[impl PartialOrd]) -> usize {
    input.array_windows().filter(|[a, b]| a < b).count()
}

fn part_b<'a, T>(input: &'a [T]) -> usize
where
    T: PartialOrd + Sum<&'a T>,
{
    part_a(
        &input
            .array_windows::<3>()
            .map(|w| w.iter().sum::<T>())
            .collect::<Vec<_>>(),
    )
}

fn parse(s: &str) -> Vec<i32> {
    s.lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse(include_str!("input"));

    println!("a: {}", part_a(&input));
    println!("b: {}", part_b(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = parse(
            "
199
200
208
210
200
207
240
269
260
263
",
        );

        assert_eq!(part_a(&input), 7);
        assert_eq!(part_b(&input), 5);
    }
}
