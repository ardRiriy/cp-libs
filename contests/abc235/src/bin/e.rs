use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        q: usize,
        e: [(Usize1, Usize1, u64); m],
        queries: [(Usize1, Usize1, u64); q],
    }
    let mut queries = queries.iter()
        .enumerate()
        .map(|(idx, &(u, v, w))| (0, u, v, w, idx))
        .collect_vec();
    let e = e.iter()
        .map(|&(u, v, w)| (1, u, v, w, 1<<60))
        .collect_vec();
    queries.extend(e);

    let mut ans = vec![false; q];
    let mut uf = Dsu::new(n);
    for &(t, u, v, _, idx) in queries.iter().sorted_unstable_by_key(|&x| x.3) {
        if t == 0 {
            ans[idx] = !uf.same(u, v);
        } else {
            uf.merge(u, v);
        }
    }
    println!("{}", ans.iter().map(|b| if *b { "Yes" } else { "No" }).join("\n"));
}

