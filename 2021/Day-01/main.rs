fn part_a(input: &Vec<i32>) -> usize {
    input
        .windows(2)
        .filter(|&w| match w {
            &[a, b] => a < b,
            _ => panic!(),
        })
        .count()
}

fn part_b(input: &Vec<i32>) -> usize {
    input
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|&w| match w {
            &[a, b] => a < b,
            _ => panic!(),
        })
        .count()
}

pub fn main() {
    let measurements = include_str!("input")
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("a: {:?}", part_a(&measurements));
    println!("b: {:?}", part_b(&measurements));
}
