use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [i64; n],
        e: [(Usize1, Usize1); m],
    }
    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)|{ g[u].push(v); g });

    let inf :i64 = 1 << 62;
    let mut ans :i64 = -inf;
    let mut bought_price = vec![inf; n];

    for i in 0..n {
        if bought_price[i] != inf {
            ans.chmax(a[i] - bought_price[i]);
        }
        let val = bought_price[i].min(a[i]);
        for &ni in g[i].iter() {
            bought_price[ni].chmin(val);
        }
    }
    println!("{ans}");
}

