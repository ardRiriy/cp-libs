use std::iter::repeat;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

#[derive(Clone, Copy, Debug)]
struct Node {
    p: usize,
    w_sum: u64,
    vertex_count: u64,
}

impl Node {
    fn new(p: usize) -> Self {
        Node { p, w_sum: 1, vertex_count: 1 }
    }

    fn add(&mut self, other: &Node) {
        self.w_sum += other.w_sum + other.vertex_count;
        self.vertex_count += other.vertex_count;
    }
}

fn dfs1(p: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<Vec<Node>>, seen: &mut Vec<bool>) -> Node {
    let mut node = Node::new(p);
    for &ni in g[p].iter() {
        if seen[ni] { continue; }
        seen[ni] = true;
        let child = dfs1(ni, g, dp, seen);
        node.add(&child);
        dp[p].push(child);
    }
    node
}

fn dfs2(p: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<Vec<Node>>, seen: &mut Vec<bool>, ans: &mut Vec<u64>) {
    let s = dp[p].iter().map(|nd| nd.w_sum).sum::<u64>();
    let vs = dp[p].iter().into_iter().map(|nd| nd.vertex_count).sum::<u64>();
    ans[p] = s;

    let mut base = 0;
    for (i, ni) in g[p].iter().enumerate() {
        if seen[*ni] {
            base += 1; 
            continue; 
        }
        assert_eq!(dp[p][i-base].p, *ni);
        seen[*ni] = true;
        let next_node = Node { p, w_sum: s - dp[p][i-base].w_sum + (vs - dp[p][i-base].vertex_count + 1), vertex_count: vs - dp[p][i-base].vertex_count + 1};
        dp[*ni].push(next_node);
        dfs2(*ni, g, dp, seen, ans);
    }
}


fn main() {
    input!{
        n: usize,
        e: [(Usize1, Usize1); n-1],
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    let mut dp = vec![vec![]; n];
    {
        let mut seen = repeat(false).take(n).collect::<Vec<_>>(); 
        seen[0] = true;
        dfs1(0, &g, &mut dp, &mut seen);
    }

    let mut ans = vec![0; n];
    {
        let mut seen = repeat(false).take(n).collect::<Vec<_>>(); 
        seen[0] = true;
        dfs2(0, &g, &mut dp, &mut seen, &mut ans);
    }
    println!("{}", ans.iter().join("\n"));
}

