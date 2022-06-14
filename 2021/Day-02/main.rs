fn part_a(input: &[(&str, i32)]) -> (i32, i32) {
    input
        .iter()
        .fold((0, 0), |(horz, depth), &(dir, x)| match dir {
            "forward" => (horz + x, depth),
            "up" => (horz, depth - x),
            "down" | _ => (horz, depth + x),
        })
}

fn part_b(input: &[(&str, i32)]) -> (i32, i32, i32) {
    input
        .iter()
        .fold((0, 0, 0), |(horz, depth, aim), &(dir, x)| match dir {
            "forward" => (horz + x, depth + aim * x, aim),
            "up" => (horz, depth, aim - x),
            "down" | _ => (horz, depth, aim + x),
        })
}

fn main() {
    let commands = include_str!("input")
        .lines()
        .filter_map(|l| l.split_once(" "))
        .filter_map(|(dir, x)| x.parse::<i32>().ok().map(|x| (dir, x)))
        .collect::<Vec<_>>();

    let (horz, depth) = part_a(&commands);
    println!("a: {}", horz * depth);

    let (horz, depth, _) = part_b(&commands);
    println!("b: {}", horz * depth);
}
