fn part_a(input: &[i32]) -> i32 {
    for i in &input[..input.len() - 1] {
        for j in &input[1..] {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    0
}

fn part_b(input: &[i32]) -> i32 {
    for i in &input[..input.len() - 2] {
        for j in &input[1..input.len() - 1] {
            for k in &input[2..] {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    0
}

fn main() {
    let vals = include_str!("input")
        .lines()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("a: {}", part_a(&vals));
    println!("b: {}", part_b(&vals));
}
