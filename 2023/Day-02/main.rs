use std::str;

fn part_a(input: &[(u32, u32, u32)], max: (u32, u32, u32)) -> u32 {
    input
        .iter()
        .enumerate()
        .filter(|(_, (r, _, _))| r <= &max.0)
        .filter(|(_, (_, g, _))| g <= &max.1)
        .filter(|(_, (_, _, b))| b <= &max.2)
        .map(|(id, _)| (id + 1) as u32)
        .sum::<u32>()
}

fn part_b(input: &[(u32, u32, u32)]) -> u32 {
    input.iter().map(|(r, g, b)| r * g * b).sum::<u32>()
}

fn parse(s: &str) -> Vec<(u32, u32, u32)> {
    s.lines()
        .filter_map(|l| l.split_once(':').map(|s| s.1))
        .map(|s| {
            s.split(';').fold((0, 0, 0), |acc, p| {
                let (mut r, mut g, mut b) = acc;
                for x in p.split(',') {
                    let t = x.trim();
                    let Some(t) = t.split_once(' ') else { continue };
                    let Ok(v) = t.0.parse::<u32>() else { continue };
                    match t.1 {
                        "red" => r = r.max(v),
                        "green" => g = g.max(v),
                        "blue" => b = b.max(v),
                        _ => {}
                    }
                }
                (r, g, b)
            })
        })
        .collect()
}

pub fn main() {
    let input = include_str!("input");

    println!("a: {}", part_a(&parse(input), (12, 13, 14)));
    println!("b: {}", part_b(&parse(input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = parse(
            "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );

        assert_eq!(part_a(&input, (12, 13, 14)), 8);
        assert_eq!(part_b(&input), 2286);
    }
}
