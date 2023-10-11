use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let f = File::open("../input01.txt").unwrap();
    let r = BufReader::new(f);
    let vals: Vec<i32> = r
        .lines()
        .flatten()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    'outer: for i in &vals[..vals.len() - 1] {
        for j in &vals[1..] {
            if i + j == 2020 {
                println!("{}", i * j);
                break 'outer;
            }
        }
    }
}

pub fn part2() {
    let f = File::open("../input01.txt").unwrap();
    let r = BufReader::new(f);
    let vals: Vec<i32> = r
        .lines()
        .flatten()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    'outer: for i in &vals[..vals.len() - 2] {
        for j in &vals[1..vals.len() - 1] {
            for k in &vals[2..] {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    break 'outer;
                }
            }
        }
    }
}
