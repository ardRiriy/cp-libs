// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use cps::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    
    let mut uf = UnionFind::new(n, |a: &usize, _: &usize| *a);
    
    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }
        
        match t {
            0 => {
                uf.merge(u, v);
            },
            1 => {
                println!("{}", if uf.same(u, v) { 1 } else { 0 });
            }
            _ => unreachable!()
        }
    }
}
