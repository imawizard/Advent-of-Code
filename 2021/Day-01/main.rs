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
            .map(|w| w.into_iter().sum::<T>())
            .collect::<Vec<_>>(),
    )
}

pub fn main() {
    let measurements = include_str!("input")
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("a: {}", part_a(&measurements));
    println!("b: {}", part_b(&measurements));
}
