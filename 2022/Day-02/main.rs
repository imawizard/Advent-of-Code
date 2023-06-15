fn part_a(input: &[(i32, i32)]) -> i32 {
    input
        .iter()
        .map(|&(a, b)| {
            if a == b {
                b + 3
            } else if a < b && a - b != -2 || a - b == 2 {
                b + 6
            } else {
                b + 0
            }
        })
        .sum()
}

fn part_b(input: &[(i32, i32)]) -> i32 {
    part_a(
        &input
            .iter()
            .map(|&(a, b)| {
                (
                    a,
                    match b {
                        1 => ((a + 1) % 3) + 1,
                        2 => a,
                        _ => (a % 3) + 1,
                    },
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn main() {
    let input = include_str!("input")
        .lines()
        .filter_map(|l| l.split_once(" "))
        .map(|(a, b)| {
            (
                (a.as_bytes()[0] - b'A' + 1) as i32,
                (b.as_bytes()[0] - b'X' + 1) as i32,
            )
        })
        .collect::<Vec<_>>();

    println!("a: {:?}", part_a(&input));
    println!("b: {:?}", part_b(&input));
}
