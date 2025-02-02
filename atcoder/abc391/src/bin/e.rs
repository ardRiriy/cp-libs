#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

struct Node {
    color: char,
    cost: i32,
}

impl Node {
    fn new(color: char, cost: i32) -> Self {
        Self { color, cost }
    }

    fn vote(v: &Vec<Node>) -> (char, i32) {
        let col = if v.iter().filter(|c| c.color == '0').count() >= 2 {
            '0'
        } else {
            '1'
        };
        let other = if col == '0' {
            '1'
        } else {
            '0'
        };
        let c = col;
        let cost = v.iter()
            .map(|c| if c.color == other { 0 } else { c.cost })
            .sorted_unstable()
            .enumerate()
            .map(|(i, c)| if i == 2 { 0 } else { c })
            .sum::<i32>();
        (c, cost)
    }
}

fn dfs(i: usize, n: usize, offset: usize, s: &Vec<char>) -> Node {
    if i == n {
        return Node::new(s[offset], 1);
    }

    let k = s.len() / 3usize.pow(i as u32+1);
    let mut v = vec![];
    for j in 0..3 {
        v.push(dfs(i+1, n, offset + k*j, s));
    }
    let (c, cost) = Node::vote(&v);
    Node::new(c, cost)
}

fn main() {
    input!{
        n: usize,
        s: Chars,
    }
    let res = dfs(0, n, 0, &s);
    println!("{}", res.cost);
}

