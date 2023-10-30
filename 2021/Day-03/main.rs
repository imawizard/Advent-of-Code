use std::collections::HashMap;
use std::iter;

fn part_a(input: &[u32], bitcnt: u32) -> (u32, u32) {
    let mut m = HashMap::new();
    input
        .iter()
        .flat_map(|num| iter::repeat(num).zip(0..bitcnt))
        .for_each(|(num, i)| *m.entry(i).or_insert(0) += *num >> i & 1);

    let gamma = m
        .into_iter()
        .filter(|&(_, n)| n as usize >= input.len() / 2)
        .fold(0, |acc, (i, _)| acc | 1 << i);
    let epsilon = !gamma & ((1 << bitcnt) - 1);

    (gamma, epsilon)
}

fn parse(s: &str) -> (Vec<u32>, u32) {
    let numbers = s
        .lines()
        .filter_map(|v| u32::from_str_radix(v, 2).ok())
        .collect::<Vec<_>>();
    let bitcnt = u32::BITS - numbers.iter().max().unwrap().leading_zeros();

    (numbers, bitcnt)
}

fn main() {
    let (numbers, bitcnt) = parse(include_str!("input"));

    let (epsilon, gamma) = part_a(&numbers, bitcnt);
    println!("a: {}", epsilon * gamma);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (numbers, bitcnt) = parse(
            "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
",
        );

        assert_eq!(part_a(&numbers, bitcnt), (22, 9));
    }
}
