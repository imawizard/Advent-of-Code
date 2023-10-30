#![feature(array_chunks)]
#![allow(clippy::type_complexity)]

use std::collections::HashMap;
use std::iter::{repeat, zip};

fn part_a(input: &[((i32, i32), (i32, i32))]) -> usize {
    let mut m = HashMap::new();

    let (vert, horz): (Vec<(_, _)>, Vec<(_, _)>) = input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .partition(|((x1, _), (x2, _))| x1 == x2);

    vert.into_iter().for_each(|((x, y1), (_, y2))| {
        (y1.min(y2)..=y2.max(y1))
            .for_each(|y| *m.entry((x, y)).or_insert(0) += 1)
    });

    horz.into_iter().for_each(|((x1, y), (x2, _))| {
        (x1.min(x2)..=x2.max(x1))
            .for_each(|x| *m.entry((x, y)).or_insert(0) += 1)
    });

    m.into_values().filter(|&v| v >= 2).count()
}

fn part_b(input: &[((i32, i32), (i32, i32))]) -> usize {
    let mut m = HashMap::new();

    let (vert, rest): (Vec<(_, _)>, Vec<(_, _)>) =
        input.iter().partition(|((x1, _), (x2, _))| x1 == x2);
    let (horz, diag): (Vec<(_, _)>, Vec<(_, _)>) =
        rest.into_iter().partition(|((_, y1), (_, y2))| y1 == y2);

    diag.into_iter()
        .flat_map(|((x1, y1), (x2, y2))| [(x1, x2), (y1, y2)])
        .map(|(a, b)| {
            if a < b {
                (a..=b).collect::<Vec<_>>()
            } else {
                (b..=a).rev().collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>()
        .array_chunks()
        .flat_map(|[a, b]| zip(a, b))
        .map(|(&x, &y)| (x, y))
        .chain(vert.into_iter().flat_map(|((x, y1), (_, y2))| {
            repeat(x).zip(y1.min(y2)..=y2.max(y1))
        }))
        .chain(horz.into_iter().flat_map(|((x1, y), (x2, _))| {
            (x1.min(x2)..=x2.max(x1)).zip(repeat(y))
        }))
        .for_each(|(x, y)| *m.entry((x, y)).or_insert(0) += 1);

    m.into_values().filter(|&v| v >= 2).count()
}

fn parse(s: &str) -> Vec<((i32, i32), (i32, i32))> {
    s.lines()
        .filter_map(|l| l.split_once(" -> "))
        .flat_map(|(p1, p2)| [p1, p2])
        .map(|p| p.split_once(',').unwrap())
        .flat_map(|(x, y)| [x, y])
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .array_chunks()
        .map(|&[x, y]| (x, y))
        .collect::<Vec<_>>()
        .array_chunks()
        .map(|&[p1, p2]| (p1, p2))
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
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
",
        );

        assert_eq!(part_a(&input), 5);
        assert_eq!(part_b(&input), 12);
    }
}
