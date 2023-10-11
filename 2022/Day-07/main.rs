use std::collections::HashMap;

fn part_a(input: Vec<Input>) -> i32 {
    //let mut paths = HashMap::new();

    for x in input.iter() {
        //
        match x {
            Input::Cmd(s) => {
                if let Some((_, path)) = s.split_once(" ") {
                    println!("{:?}", path);
                }
            }
            Input::Res(s) => {
                for x in s.lines().map(|s| s.split_once(" ").unwrap()) {
                    println!("{:?} {:?}", x.0, x.1);
                }
            }
        }
    }

    let mut cwd = "";
    0
}

#[derive(Debug)]
enum Input<'a> {
    Cmd(&'a str),
    Res(&'a str),
}

fn parse(s: &str) -> Vec<Input> {
    s.lines()
        .map(|l| match l.trim() {
            cmd if cmd.starts_with("$") => Input::Cmd(&cmd[2..]),
            res => Input::Res(res),
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse(include_str!("input"));

    println!("a: {:?}", part_a(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        assert_eq!(
            part_a(parse(
                "$ cd /\
                 $ ls\
                 dir a\
                 14848514 b.txt\
                 8504156 c.dat\
                 dir d\
                 $ cd a\
                 $ ls\
                 dir e\
                 29116 f\
                 2557 g\
                 62596 h.lst\
                 $ cd e\
                 $ ls\
                 584 i\
                 $ cd ..\
                 $ cd ..\
                 $ cd d\
                 $ ls\
                 4060174 j\
                 8033020 d.log\
                 5626152 d.ext\
                 7214296 k",
            )),
            95437,
        );
    }
}
