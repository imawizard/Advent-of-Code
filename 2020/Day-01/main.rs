fn part_a(input: &[i32]) -> i32 {
    for i in &input[..input.len() - 1] {
        for j in &input[1..] {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    0
}

fn part_b(input: &[i32]) -> i32 {
    for i in &input[..input.len() - 2] {
        for j in &input[1..input.len() - 1] {
            for k in &input[2..] {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    0
}

fn parse(s: &str) -> Vec<i32> {
    s.lines()
        .filter_map(|v| v.parse::<i32>().ok())
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
1721
979
366
299
675
1456
",
        );

        assert_eq!(part_a(&input), 514579);
        assert_eq!(part_b(&input), 241861950);
    }
}
