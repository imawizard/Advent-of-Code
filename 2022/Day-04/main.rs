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

fn parse(s: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    s.lines()
        .filter_map(|l| l.split_once(','))
        .map(|(f, s)| {
            [f, s]
                .iter()
                .map(|v| v.split_once('-').unwrap())
                .map(|(s, e)| {
                    (s.parse::<i32>().unwrap(), e.parse::<i32>().unwrap())
                })
                .map(|(s, e)| s..=e)
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0].clone(), v[1].clone()))
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
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
",
        );

        assert_eq!(part_a(&input), 2);
        assert_eq!(part_b(&input), 4);
    }
}
