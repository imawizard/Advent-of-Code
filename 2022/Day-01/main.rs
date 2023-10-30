fn part_a(parts: &[Vec<u32>]) -> Option<u32> {
    let sums = parts
        .iter()
        .map(|l| l.iter().sum::<u32>())
        .collect::<Vec<_>>();
    sums.iter().max().copied()
}

fn part_b(parts: &[Vec<u32>], n: u32) -> u32 {
    let mut sums = parts
        .iter()
        .map(|l| l.iter().sum::<u32>())
        .collect::<Vec<_>>();
    sums.sort();
    sums[sums.len() - n as usize..].iter().sum()
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .collect::<Vec<_>>()
        .split(|&l| l.is_empty())
        .map(|l| {
            l.iter()
                .filter_map(|v| v.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse(include_str!("input"));

    println!("a: {}", part_a(&input).unwrap());
    println!("b: {}", part_b(&input, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = parse(
            "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
",
        );

        assert_eq!(part_a(&input), Some(24000));
        assert_eq!(part_b(&input, 3), 45000);
    }
}
