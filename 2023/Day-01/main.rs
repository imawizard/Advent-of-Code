use std::str;

fn parse_a(s: &str) -> Vec<u32> {
    s.lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>())
        .filter(|a| !a.is_empty())
        .map(|a| (*a.first().unwrap(), *a.last().unwrap()))
        .map(|(a, b)| (a.to_digit(10).unwrap(), b.to_digit(10).unwrap()))
        .map(|(a, b)| a * 10 + b)
        .collect()
}

fn parse_b(s: &str) -> Vec<u32> {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut s = s.bytes().collect::<Vec<_>>();
    let mut i = 0;
    while i < s.len() {
        for (j, d) in digits.into_iter().enumerate() {
            if s[i..].starts_with(d.as_bytes()) {
                s[i] = b'1' + j as u8;
                // NOTE: Instructions are kind of imprecise: spelled out digits
                // are allowed to overlap, thus "one" is not equivalent "1" and
                // "oneight" doesn't result in "1ight" but rather "18".
                //s[i + 1..].copy_within(d.len() - 1.., 0);
                //s.truncate(s.len() - d.len() + 1);
                break;
            }
        }
        i += 1;
    }
    parse_a(str::from_utf8(&s).unwrap())
}

pub fn main() {
    let input = include_str!("input");

    println!("a: {}", parse_a(input).iter().sum::<u32>());
    println!("b: {}", parse_b(input).iter().sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = parse_a(
            "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
        );

        assert_eq!(input.iter().sum::<u32>(), 142);
    }

    #[test]
    fn example_b() {
        let input = parse_b(
            "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );

        assert_eq!(input.iter().sum::<u32>(), 281);
    }
}
