use ac_library::Dsu;
use cps::chlibs::ChLibs;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        e: [(Usize1, Usize1, u64); m],
    }

    let mut ans = 1<<60;
    for v in (0..m).into_iter().combinations(n-1) {
        let mut uf = Dsu::new(n);
        let mut sum = 0;
        for i in v {
            uf.merge(e[i].0, e[i].1); 
            sum += e[i].2;
        }
        if uf.size(0) == n {
            ans.chmin(sum%k);
        }
    }
    println!("{}", ans);
}
