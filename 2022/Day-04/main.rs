use std::ops::RangeInclusive;

fn part_a(input: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> usize {
    input
        .iter()
        .filter(|(s, e)| {
            s.clone().all(|v| e.contains(&v))
                || e.clone().all(|v| s.contains(&v))
        })
        .count()
}

fn part_b(input: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> usize {
    input
        .iter()
        .filter(|(s, e)| {
            s.clone().any(|v| e.contains(&v))
                || e.clone().any(|v| s.contains(&v))
        })
        .count()
}

fn main() {
    let input = include_str!("input")
        .lines()
        .filter_map(|l| l.split_once(","))
        .map(|(f, s)| {
            [f, s]
                .iter()
                .map(|v| v.split_once("-").unwrap())
                .map(|(s, e)| {
                    (s.parse::<i32>().unwrap(), e.parse::<i32>().unwrap())
                })
                .map(|(s, e)| s..=e)
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0].clone(), v[1].clone()))
        .collect::<Vec<_>>();

    println!("a: {}", part_a(&input));
    println!("b: {}", part_b(&input));
}
