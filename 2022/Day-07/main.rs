use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn part_a(input: &[Entry]) -> usize {
    collect_sizes(input.iter())
        .into_iter()
        .filter(|(_, s)| *s <= 100_000)
        .map(|(_, s)| s)
        .sum()
}

fn part_b(input: &[Entry]) -> usize {
    let sizes = collect_sizes(input.iter());
    let total = 70_000_000;
    let used = sizes[Path::new("/")];
    let unused = total - used;
    let needed = 30_000_000;

    sizes
        .into_iter()
        .filter(|(_, s)| *s + unused >= needed)
        .map(|(_, s)| s)
        .min()
        .unwrap()
}

fn collect_sizes<'a>(
    entries: impl Iterator<Item = &'a Entry<'a>>,
) -> HashMap<PathBuf, usize> {
    let mut children = HashMap::new();
    let mut cwd = PathBuf::from("/");

    for e in entries {
        let mut i = e.cmd.split(' ');

        match i.next().unwrap() {
            "cd" => match i.next().unwrap() {
                ".." => {
                    cwd.pop();
                }
                path => {
                    cwd.push(path);
                }
            },
            "ls" => {
                children.entry(cwd.clone()).or_insert(
                    e.out
                        .unwrap()
                        .lines()
                        .filter_map(|l| l.split_once(' '))
                        .map(|(v, k)| (k, v))
                        .collect::<HashMap<_, _>>(),
                );
            }
            _ => {}
        }
    }

    fn fill_sizes<'a>(
        acc: &mut HashMap<PathBuf, usize>,
        path: PathBuf,
        children: &HashMap<PathBuf, HashMap<&'a str, &'a str>>,
    ) -> usize {
        let size = children[&path]
            .iter()
            .map(|(name, size)| {
                size.parse::<usize>().unwrap_or_else(|_| {
                    fill_sizes(acc, path.join(name), children)
                })
            })
            .sum();
        *acc.entry(path).or_insert(size)
    }

    let mut sizes = HashMap::new();
    fill_sizes(&mut sizes, PathBuf::from("/"), &children);
    sizes
}

#[derive(Debug)]
struct Entry<'a> {
    cmd: &'a str,
    out: Option<&'a str>,
}

fn parse(s: &str) -> Vec<Entry> {
    s.split("\n$ ")
        .map(|p| match p.split_once('\n') {
            Some((a, b)) => Entry {
                cmd: a,
                out: Some(b),
            },
            None => Entry { cmd: p, out: None },
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse(include_str!("input"));

    println!("a: {}", part_a(&input));
    println!("b: {}", part_b(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = parse(
            "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
",
        );

        assert_eq!(part_a(&input), 95437);
        assert_eq!(part_b(&input), 24933642);
    }
}
