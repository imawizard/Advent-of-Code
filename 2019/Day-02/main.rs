fn part_a(input: &[usize], change: Option<(usize, usize)>) -> Vec<usize> {
    let mut prog = input.to_vec();
    if let Some((noun, verb)) = change {
        prog[1] = noun;
        prog[2] = verb;
    }
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
        .find(|&(noun, verb)| {
            part_a(input, Some((noun, verb)))[0] == wanted_output
        })
}

fn parse(s: &str) -> Vec<usize> {
    s.split(',')
        .filter_map(|op| op.trim().parse::<usize>().ok())
        .collect::<Vec<_>>()
}

pub fn main() {
    let opcodes = parse(include_str!("input"));

    println!("a: {:?}", part_a(&opcodes, Some((12, 2)))[0]);
    println!(
        "b: {:?}",
        part_b(&opcodes, 19690720).map(|(noun, verb)| 100 * noun + verb)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input1 = parse("1,0,0,0,99");
        let input2 = parse("2,3,0,3,99");
        let input3 = parse("2,4,4,5,99,0");
        let input4 = parse("1,1,1,4,99,5,6,0,99");

        assert_eq!(part_a(&input1, None), vec![2, 0, 0, 0, 99]);
        assert_eq!(part_a(&input2, None), vec![2, 3, 0, 6, 99]);
        assert_eq!(part_a(&input3, None), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(part_a(&input4, None), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
