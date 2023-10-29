fn part_a(input: &[Vec<u32>]) -> usize {
    let width = input[0].len();
    let height = input.len();
    let mut count = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let v = input[y][x];

            if (0..x).all(|x| input[y][x] < v)
                || (x + 1..width).all(|x| input[y][x] < v)
                || (0..y).all(|y| input[y][x] < v)
                || (y + 1..height).all(|y| input[y][x] < v)
            {
                count += 1;
            }
        }
    }
    count + width * 2 + (height - 2) * 2
}

fn part_b(input: &[Vec<u32>]) -> Option<usize> {
    let width = input[0].len();
    let height = input.len();
    let mut scenic_scores = Vec::new();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let v = input[y][x];
            let mut left =
                (0..x).rev().take_while(|&x| input[y][x] < v).count();
            let mut right =
                (x + 1..width).take_while(|&x| input[y][x] < v).count();
            let mut up = (0..y).rev().take_while(|&y| input[y][x] < v).count();
            let mut down =
                (y + 1..height).take_while(|&y| input[y][x] < v).count();
            if left < x {
                left += 1
            }
            if right < width - x - 1 {
                right += 1
            }
            if up < y {
                up += 1
            }
            if down < height - y - 1 {
                down += 1
            }
            scenic_scores.push(left * right * up * down);
        }
    }
    scenic_scores.into_iter().max()
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn main() {
    let input = parse(include_str!("input"));

    println!("a: {}", part_a(&input));
    println!("b: {}", part_b(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = parse(
            "
30373
25512
65332
33549
35390
",
        );
        assert_eq!(part_a(&input), 21);
        assert_eq!(part_b(&input), Some(8));
    }
}
