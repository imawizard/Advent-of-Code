fn part_a(input: &[usize], (noun, verb): (usize, usize)) -> Vec<usize> {
    let mut prog = input.to_vec();
    prog[1] = noun;
    prog[2] = verb;
    let mut pc = 0;
    loop {
        match prog[pc] {
            op @ (1 | 2) => {
                let pos1 = prog[pc + 1];
                let pos2 = prog[pc + 2];
                let dest = prog[pc + 3];
                prog[dest] = match op {
                    1 => prog[pos1] + prog[pos2],
                    _ => prog[pos1] * prog[pos2],
                };
            }
            99 => break,
            _ => panic!(),
        }
        pc += 4;
    }
    prog
}

fn part_b(input: &[usize], wanted_output: usize) -> Option<(usize, usize)> {
    let range = 0..=99;
    range
        .clone()
        .flat_map(|noun| vec![noun; *range.end()])
        .zip(range.clone().cycle())
        .find(|&(noun, verb)| part_a(input, (noun, verb))[0] == wanted_output)
}

pub fn main() {
    let opcodes = include_str!("input")
        .split(",")
        .filter_map(|op| op.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("a: {:?}", part_a(&opcodes, (12, 2))[0]);
    println!(
        "b: {:?}",
        match part_b(&opcodes, 19690720) {
            Some((noun, verb)) => 100 * noun + verb,
            _ => panic!(),
        }
    );
}
