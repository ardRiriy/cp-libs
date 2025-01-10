use cps::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        a: [Usize1; n],
    }
    static N: usize = 2e5 as usize;
    let mut ans = 0;
    let mut uf = UnionFind::new(N, |x: &i32, _| *x);
    for i in 0..n/2 {
        let j = n-i-1;
        if uf.merge(a[i], a[j]) {
            ans += 1u64;
        }
    }
    println!("{}", ans);
}

