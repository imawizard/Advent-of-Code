#![feature(array_chunks)]

use std::collections::HashSet;

fn part_a(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(f, s)| {
            [f, s]
                .iter()
                .map(|a| a.into_iter().collect::<HashSet<_>>())
                .reduce(|acc, s| {
                    acc.intersection(&s).cloned().collect::<HashSet<_>>()
                })
                .unwrap()
        })
        .map(|s| s.into_iter().map(|v| *v as u32).sum::<u32>())
        .sum::<u32>()
}

fn part_b(input: &[Vec<u8>]) -> u32 {
    input
        .array_chunks::<3>()
        .map(|x| {
            x.into_iter()
                .map(|a| a.into_iter().collect::<HashSet<_>>())
                .reduce(|acc, s| {
                    acc.intersection(&s).cloned().collect::<HashSet<_>>()
                })
                .unwrap()
        })
        .map(|s| s.into_iter().map(|v| *v as u32).sum::<u32>())
        .sum::<u32>()
}

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.chars()
                .map(|c| match c.is_lowercase() {
                    true => c as u8 - b'a' + 1,
                    false => c as u8 - b'A' + 1 + 26,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("a: {:?}", part_a(&input));
    println!("b: {:?}", part_b(&input));
}
