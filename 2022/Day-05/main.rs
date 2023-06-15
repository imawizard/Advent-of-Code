#![feature(iter_array_chunks)]

use std::collections::HashMap;

fn part_a(stacks: &[Vec<char>], input: &[(usize, usize, usize)]) -> String {
    let mut state: Vec<Vec<char>> = Vec::new();
    stacks.iter().for_each(|s| state.push(s.clone()));

    input.iter().for_each(|x| {
        (0..x.0).into_iter().for_each(|_| {
            let tmp = state[x.1 - 1].pop().unwrap();
            state[x.2 - 1].push(tmp);
        })
    });

    state
        .into_iter()
        .filter_map(|s| s.last().cloned().map(String::from))
        .collect::<Vec<_>>()
        .join("")
}

fn part_b(stacks: &[Vec<char>], input: &[(usize, usize, usize)]) -> String {
    let mut state: Vec<Vec<char>> = Vec::new();
    stacks.iter().for_each(|s| state.push(s.clone()));

    input.iter().for_each(|x| {
        let len = state[x.1 - 1].len();
        let removed = state[x.1 - 1]
            .splice((len - x.0)..len, [])
            .collect::<Vec<_>>();

        removed.into_iter().for_each(|tmp| state[x.2 - 1].push(tmp))
    });

    state
        .into_iter()
        .filter_map(|s| s.last().cloned().map(String::from))
        .collect::<Vec<_>>()
        .join("")
}

fn main() {
    let (stacks, input) = include_str!("input").split_once("\n\n").unwrap();

    let stacks = stacks
        .lines()
        .map(|l| format!("{} ", l))
        .filter(|l| l.contains('['))
        .map(|l| {
            l.chars()
                .array_chunks::<4>()
                .map(|p| p[1])
                .collect::<Vec<_>>()
        })
        .fold(HashMap::<usize, Vec<char>>::new(), |mut acc, l| {
            l.iter().enumerate().for_each(|(i, &v)| {
                acc.entry(i).or_insert(Vec::new()).push(v);
            });
            acc
        });

    let stacks = (0..stacks.len())
        .into_iter()
        .map(|i| {
            stacks[&i]
                .iter()
                .filter(|v| !v.is_ascii_whitespace())
                .rev()
                .cloned()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let input = input
        .lines()
        .map(|l| {
            l.split_terminator(|c: char| !c.is_ascii_digit())
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1], v[2]))
        .collect::<Vec<_>>();

    println!("a: {:?}", part_a(&stacks, &input));
    println!("b: {:?}", part_b(&stacks, &input));
}
