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
                            let range: Box<dyn Iterator<Item = _>> = if old_x < *x {
                                Box::new(old_x..=*x)
                            } else {
                                Box::new((*x..=old_x).rev())
                            };
                            Some(range.zip(iter::repeat(*y)).collect::<Vec<_>>())
                        }
                        (0, delta) => {
                            let old_y = *y;
                            *y += delta;
                            let range: Box<dyn Iterator<Item = _>> = if old_y < *y {
                                Box::new(old_y..=*y)
                            } else {
                                Box::new((*y..=old_y).rev())
                            };
                            Some(iter::repeat(*x).zip(range).collect::<Vec<_>>())
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

fn part_b(input: &[Vec<&str>]) -> Option<usize> {
    let wires = input
        .iter()
        .map(|path| {
            path.iter()
                .scan((0, 0), |(x, y), m| {
                    let mut chars = m.chars();
                    let dir = chars.next().unwrap();
                    let delta = chars.as_str().parse::<i32>().unwrap();
                    let (mut x_delta, mut y_delta) = (0, 0);
                    match dir {
                        'R' => x_delta = delta,
                        'L' => x_delta = -delta,
                        'U' => y_delta = delta,
                        'D' => y_delta = -delta,
                        _ => return None,
                    };
                    let range: Box<dyn Iterator<Item = _>>;
                    match (x_delta, y_delta) {
                        (delta, 0) => {
                            let old_x = *x;
                            *x += delta;
                            range = if old_x < *x {
                                Box::new(old_x..=*x)
                            } else {
                                Box::new((*x..=old_x).rev())
                            };
                            Some(range.skip(1).zip(iter::repeat(*y)).collect::<Vec<_>>())
                        }
                        (0, delta) => {
                            let old_y = *y;
                            *y += delta;
                            range = if old_y < *y {
                                Box::new(old_y..=*y)
                            } else {
                                Box::new((*y..=old_y).rev())
                            };
                            Some(iter::repeat(*x).zip(range.skip(1)).collect::<Vec<_>>())
                        }
                        _ => None,
                    }
                })
                .flatten()
                // .collect::<Vec<_>>()
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    let steps_to_intersect = wires[0]
        .iter()
        .enumerate()
        .filter_map(|(i, &pt1)| {
            wires[1]
                .iter()
                .enumerate()
                .find(|(_, &pt2)| pt2 == pt1)
                .map(|(j, _)| (i + 1, j + 1))
        })
        .collect::<Vec<_>>();

    steps_to_intersect.iter().map(|(a, b)| a + b).min()
}

pub fn main() {
    let paths = include_str!("input")
        .lines()
        .map(|line| line.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("a: {:?}", part_a(&paths));
    println!("b: {:?}", part_b(&paths));
}
