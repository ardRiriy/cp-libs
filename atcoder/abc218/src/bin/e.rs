use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1, i64); m]        
    }

    let mut uf = Dsu::new(n);
    let mut ans = 0;
    for &(u, v, w) in e.iter().sorted_by_key(|&(_, _, w)| *w) {
        if w <= 0 || !uf.same(u, v) {
            uf.merge(u, v);
        } else {
            ans += w;
        }
    }
    println!("{}", ans);
}