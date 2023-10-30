fn part_a(input: &[i32]) -> i32 {
    input.iter().map(|mass| mass / 3 - 2).sum()
}

fn part_b(input: &[i32]) -> i32 {
    input.iter().copied().flat_map(calc_fuel).sum()
}

fn calc_fuel(mass: i32) -> Vec<i32> {
    let mut a = vec![];
    let mut rem = mass;
    loop {
        rem = rem / 3 - 2;
        if rem <= 0 {
            break;
        }
        a.push(rem);
    }
    a
}

pub fn main() {
    let modules = include_str!("input")
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("a: {}", part_a(&modules));
    println!("b: {}", part_b(&modules));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_a(&[12]), 2);
        assert_eq!(part_a(&[14]), 2);
        assert_eq!(part_a(&[1969]), 654);
        assert_eq!(part_a(&[100756]), 33583);

        assert_eq!(part_b(&[12]), 2);
        assert_eq!(part_b(&[14]), 2);
        assert_eq!(part_b(&[1969]), 966);
        assert_eq!(part_b(&[100756]), 50346);
    }
}
