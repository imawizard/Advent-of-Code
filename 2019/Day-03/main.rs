use std::collections::{HashMap, HashSet};
use std::iter;

fn part_a(input: &[Vec<&str>]) -> Option<i32> {
    let wires = input
        .iter()
        .map(|path| {
            path.iter()
                .scan((0, 0), |(ref mut x, ref mut y), m| {
                    let mut chars = m.chars();
                    let dir = chars.next().unwrap();
                    let delta = chars.as_str().parse::<i32>().unwrap();

                    let x_delta = match dir {
                        'R' => delta,
                        'L' => -delta,
                        _ => 0,
                    };

                    let y_delta = match dir {
                        'U' => delta,
                        'D' => -delta,
                        _ => 0,
                    };

                    match (x_delta, y_delta) {
                        (delta, 0) => {
                            let old_x = *x;
                            *x += delta;
                            let range: Box<dyn Iterator<Item = _>> =
                                if old_x < *x {
                                    Box::new(old_x..=*x)
                                } else {
                                    Box::new((*x..=old_x).rev())
                                };
                            Some(
                                range.zip(iter::repeat(*y)).collect::<Vec<_>>(),
                            )
                        }
                        (0, delta) => {
                            let old_y = *y;
                            *y += delta;
                            let range: Box<dyn Iterator<Item = _>> =
                                if old_y < *y {
                                    Box::new(old_y..=*y)
                                } else {
                                    Box::new((*y..=old_y).rev())
                                };
                            Some(
                                iter::repeat(*x).zip(range).collect::<Vec<_>>(),
                            )
                        }
                        _ => None,
                    }
                })
                .flatten()
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    let mut combined = HashMap::new();
    for wire in wires.iter() {
        for coords in wire.iter() {
            *combined.entry(coords).or_insert(0) += 1;
        }
    }

    combined
        .into_iter()
        .filter(|(_, v)| *v > 1)
        .filter(|(&(x, y), _)| x != 0 && y != 0)
        .map(|(&(x, y), _)| x.abs() + y.abs())
        .min()
}

fn parse(s: &str) -> Vec<Vec<&str>> {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| line.split(',').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn main() {
    let paths = parse(include_str!("input"));

    println!("a: {:?}", part_a(&paths));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input1 = parse(
            "
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
",
        );
        let input2 = parse(
            "
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
",
        );

        assert_eq!(part_a(&input1), Some(159));
        assert_eq!(part_a(&input2), Some(135));
    }
}
