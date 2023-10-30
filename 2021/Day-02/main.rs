fn part_a(input: &[(&str, i32)]) -> (i32, i32) {
    input
        .iter()
        .fold((0, 0), |(horz, depth), &(dir, x)| match dir {
            "forward" => (horz + x, depth),
            "up" => (horz, depth - x),
            "down" => (horz, depth + x),
            _ => panic!(),
        })
}

fn part_b(input: &[(&str, i32)]) -> (i32, i32, i32) {
    input
        .iter()
        .fold((0, 0, 0), |(horz, depth, aim), &(dir, x)| match dir {
            "forward" => (horz + x, depth + aim * x, aim),
            "up" => (horz, depth, aim - x),
            "down" => (horz, depth, aim + x),
            _ => panic!(),
        })
}

fn parse(s: &str) -> Vec<(&str, i32)> {
    s.lines()
        .filter_map(|l| l.split_once(' '))
        .filter_map(|(dir, x)| x.parse::<i32>().ok().map(|x| (dir, x)))
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse(include_str!("input"));

    let (horz, depth) = part_a(&input);
    println!("a: {}", horz * depth);

    let (horz, depth, _) = part_b(&input);
    println!("b: {}", horz * depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = parse(
            "
forward 5
down 5
forward 8
up 3
down 8
forward 2
",
        );

        assert_eq!(part_a(&input), (15, 10));
        assert_eq!(part_b(&input), (15, 60, 10));
    }
}
