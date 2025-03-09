use cps::consts::INF;
use cps::veclibs::VecLibs;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [(Usize1, u64); q],
    }

    let mut ans = vec![0; q];
    let mut qi = 0;
    let indicates = (0..q).sorted_unstable_by_key(|i| queries[*i].0).collect_vec();

    let mut dp = vec![INF; n];
    let mut mv = vec![INF; n];

    for i in 0..n {
        let idx = dp.lower_bound(a[i]);
        dp[idx] = a[i];
        mv[idx] = mv[idx].min(a[i]);

        while qi < q && queries[indicates[qi]].0 == i {
            let j = mv.lower_bound(queries[indicates[qi]].1);
            ans[indicates[qi]] = if j < n && mv[j] == queries[indicates[qi]].1 { j + 1 } else { j };
            qi += 1;
        }
    }
    println!("{}", ans.iter().join("\n"));
}
