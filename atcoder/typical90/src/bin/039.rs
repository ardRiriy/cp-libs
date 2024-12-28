use proconio::{input, marker::Usize1};

fn dfs(
    p: usize, 
    g: &Vec<Vec<usize>>,
    subnode_size: &mut Vec<u64>,
    seen: &mut Vec<bool>,
    ans: &mut u64,
) {
    subnode_size[p] = 1;
    for &ni in &g[p] {
        if seen[ni] {
            continue;
        }
        seen[ni] = true;
        dfs(ni, g, subnode_size, seen, ans);
        subnode_size[p] += subnode_size[ni];
        *ans += (g.len() as u64 - subnode_size[ni]) * subnode_size[ni];
    }
}

fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1); n-1],
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| { g[u].push(v); g[v].push(u); g });
    let mut subnode_size = vec![0; n];
    let mut seen  =vec![false; n];
    seen[0] = true;
    let mut ans = 0;
    dfs(0, &g, &mut subnode_size, &mut seen, &mut ans);
    println!("{}", ans);

}
