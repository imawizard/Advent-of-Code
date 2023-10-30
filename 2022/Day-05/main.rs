#![feature(iter_array_chunks)]

use std::collections::HashMap;

fn part_a(stacks: &[Vec<char>], input: &[(usize, usize, usize)]) -> String {
    let mut state: Vec<Vec<char>> = Vec::new();
    stacks.iter().for_each(|s| state.push(s.clone()));

    input.iter().for_each(|x| {
        (0..x.0).for_each(|_| {
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

#[allow(clippy::type_complexity)]
fn parse(s: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let (stacks, input) = s.split_once("\n\n").unwrap();

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
                acc.entry(i).or_default().push(v);
            });
            acc
        });

    let stacks = (0..stacks.len())
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

    (stacks, input)
}

fn main() {
    let (stacks, input) = parse(include_str!("input"));

    println!("a: {}", part_a(&stacks, &input));
    println!("b: {}", part_b(&stacks, &input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (stacks, input) = parse(
            "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
",
        );

        assert_eq!(part_a(&stacks, &input), "CMZ");
        assert_eq!(part_b(&stacks, &input), "MCD");
    }
}
