use std::collections::HashMap;
use std::iter;
use std::iter::Repeat;

fn part_a(input: &[u32], digits: u32) -> (u32, u32) {
    let mut m = HashMap::new();
    input
        .into_iter()
        .flat_map(|num| iter::repeat(num).zip(0..digits))
        .for_each(|(num, i)| *m.entry(i).or_insert(0) += *num >> i & 1);

    let gamma = m
        .into_iter()
        .filter(|&(_, n)| n as usize >= input.len() / 2)
        .fold(0, |acc, (i, _)| acc | 1 << i);
    let epsilon = !gamma & ((1 << digits) - 1);

    (gamma, epsilon)
}

fn part_b(input: &[u32], digits: u32) -> (u32, u32) {
    (0, 0)
}

fn main() {
    let numbers = include_str!("input")
        .lines()
        .filter_map(|v| u32::from_str_radix(v, 2).ok())
        .collect::<Vec<_>>();
    let digits = u32::BITS - numbers.iter().max().unwrap().leading_zeros();

    let (epsilon, gamma) = part_a(&numbers, digits);
    println!("a: {}", epsilon * gamma);

    let (epsilon, gamma) = part_b(&numbers, digits);
    println!("b: {}", epsilon * gamma);
}
