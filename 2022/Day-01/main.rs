fn part_a(parts: &[&[u32]]) -> Option<u32> {
    let sums = parts
        .iter()
        .map(|l| l.iter().sum::<u32>())
        .collect::<Vec<_>>();
    sums.iter().max().map(|x| *x)
}

fn part_b(parts: &[&[u32]], n: u32) -> u32 {
    let mut sums = parts
        .iter()
        .map(|l| l.iter().sum::<u32>())
        .collect::<Vec<_>>();
    sums.sort();
    sums[sums.len() - n as usize..].into_iter().sum::<u32>()
}

fn main() {
    let lines = include_str!("input").lines().collect::<Vec<_>>();
    let parts = lines
        .split(|&l| l.is_empty())
        .map(|l| {
            l.iter()
                .filter_map(|v| v.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "a: {}",
        part_a(&parts.iter().map(|p| p.as_slice()).collect::<Vec<_>>())
            .unwrap()
    );
    println!(
        "b: {}",
        part_b(&parts.iter().map(|p| p.as_slice()).collect::<Vec<_>>(), 3)
    );
}
