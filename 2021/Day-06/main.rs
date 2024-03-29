fn part_a(input: &[u8]) -> usize {
    (0..80)
        .fold(input.to_vec(), |acc, _| {
            acc.into_iter()
                .flat_map(|v| if v > 0 { vec![v - 1] } else { vec![6, 8] })
                .collect()
        })
        .len()
}

fn part_b(input: &[u8]) -> usize {
    let m = input
        .iter()
        .map(|&v| v as usize)
        .fold(vec![0; 9], |acc, i| {
            acc.into_iter()
                .enumerate()
                .map(|(j, v)| v + (i == j) as usize)
                .collect()
        });

    (0..256)
        .fold(m, |mut m, _| {
            m.rotate_left(1);
            m[6] += m[8];
            m
        })
        .into_iter()
        .sum()
}

fn parse(s: &str) -> Vec<u8> {
    s.trim()
        .split(',')
        .filter_map(|v| v.parse::<u8>().ok())
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
        let input = parse("3,4,3,1,2");

        assert_eq!(part_a(&input), 5934);
        assert_eq!(part_b(&input), 26984457539);
    }
}
