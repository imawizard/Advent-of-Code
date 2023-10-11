use std::collections::{HashMap, HashSet};

// fn part_a<'a>(
//     boards: &'a mut Vec<BingoBoard>,
//     input: &[u8],
// ) -> Option<(&'a BingoBoard, u8)> {
//     input
//         .iter()
//         .find_map(|&drawn| {
//             (0..boards.len())
//                 .find(|&i| boards[i].mark(drawn))
//                 .map(|i| (i, drawn))
//         })
//         .map(|(i, drawn)| (&boards[i], drawn))
// }
//
// fn part_b<'a>(
//     boards: &'a mut Vec<BingoBoard>,
//     input: &[u8],
// ) -> Option<(&'a BingoBoard, u8)> {
//     let mut done = HashSet::<usize>::new();
//
//     input
//         .iter()
//         .fold(None, |acc, &drawn| {
//             let won = (0..boards.len())
//                 .filter(|&i| !done.contains(&i))
//                 .filter(|&i| boards[i].mark(drawn))
//                 .collect::<Vec<_>>();
//
//             for &i in won.iter() {
//                 done.insert(i);
//             }
//
//             return match won.as_slice() {
//                 [first] => Some((*first, drawn)),
//                 _ => acc,
//             };
//         })
//         .map(|(i, drawn)| (&boards[i], drawn))
// }

fn part_a(boards: &[[u8; 25]], input: &[u8]) -> u32 {
    let initial = boards
        .iter()
        .map(|b| (*b, [0u8; 5], [0u8; 5]))
        .collect::<Vec<_>>();

    input.iter().fold(initial, |acc, &x| {
        acc.into_iter()
            .map(|(mut board, mut rows, mut columns)| {
                if let Some((row, col)) = board
                    .iter_mut()
                    .enumerate()
                    .find(|(_, v)| **v == x)
                    .map(|(i, _)| (i / 5, i % 5))
                {
                    board[row * 5 + col] = 0;
                    rows[row] += 1;
                    columns[col] += 1;

                    println!("{:?}", board);
                    println!("{}", rows[row]);
                    println!("{}", columns[col]);
                }
                (board, rows, columns)
            })
            .collect()
    });

    0
}

pub fn main() {
    let (input, boards) = include_str!("input").split_once("\n\n").unwrap();

    let input = input
        .split(",")
        .filter_map(|v| v.parse::<u8>().ok())
        .collect::<Vec<_>>();
    let boards = boards
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|c| {
            c.join(" ")
                .split_ascii_whitespace()
                .filter_map(|v| v.parse::<u8>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|v| v.len() == 5 * 5)
        .map(|v| {
            let mut x = [0u8; 25];
            x.copy_from_slice(&v);
            x
        })
        .collect::<Vec<_>>();

    println!("a: {}", part_a(&boards, &input));

    //     .lines()
    //     .into_iter()
    //     .collect::<Vec<_>>()
    //     .as_slice()
    // {
    //     let boards = lines
    //         .into_iter()
    //         .filter_map(|line| match line.trim() {
    //             "" => None,
    //             s @ _ => Some(s),
    //         })
    //         .collect::<Vec<_>>()
    //         .windows(5)
    //         .map(|bunch| {
    //             bunch
    //                 .join(" ")
    //                 .split_ascii_whitespace()
    //                 .map(|s| s.parse::<u8>().unwrap())
    //                 .collect::<Vec<_>>()
    //         })
    //         .filter_map(|numbers| BingoBoard::new(&numbers))
    //         .collect::<Vec<_>>();
    //
    //     let input = input
    //         .split(",")
    //         .map(|s| s.trim().parse::<u8>().unwrap())
    //         .collect::<Vec<_>>();

    // if let Some((board, drawn)) = part_a(&mut boards.clone(), &input) {
    //     println!("a: {}", board.sum() * drawn as u32);
    // }
    //
    // if let Some((board, drawn)) = part_b(&mut boards.clone(), &input) {
    //     println!("b: {}", board.sum() * drawn as u32);
    // }
    // }
}
