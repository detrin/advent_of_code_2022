use std::io::{self, BufRead};
use serde::Deserialize;
use core::cmp::Ordering;
use std::fmt;

// https://fasterthanli.me/series/advent-of-code-2022/part-13
#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (l, r) => Some(l.with_slice(|l| {
                r.with_slice(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(aa, bb)| aa.cmp(bb))
                        .find(|&ord| ord != Ordering::Equal)
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


pub fn task13_part1_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut sum = 0;
    for (i, groups) in input.split("\n\n").enumerate() {
        let i = i + 1;

        let mut nodes = groups
            .lines()
            .map(|line| serde_json::from_str::<Node>(line).unwrap());
        let l = nodes.next().unwrap();
        let r = nodes.next().unwrap();

        if l < r {
            sum += i;
        }
    }
    println!("{}", sum);

}


pub fn task13_part2_v1() {
    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec![Node::Number(6)]),
    ];

    let stdin = io::stdin();
    let mut packets = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|s| !s.is_empty())
        .map(|line| serde_json::from_str::<Node>(&line).unwrap())
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();

    packets.sort();

    let decoder_key = dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>();

    println!("{}", decoder_key);
}
    