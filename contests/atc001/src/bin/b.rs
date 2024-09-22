use cps::unionfind::UnionFind;
use proconio::input;

fn main() {
    input!{
        n: usize,
        q: usize,
    }
    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input! {
            p: u8,
            u: usize,
            v: usize,
        }

        match p {
            0 => {
                uf.merge(u, v);
            }
            1 => {
                println!("{}", if uf.same(u, v) { "Yes" } else { "No" });
            }
            _ => { unreachable!(); }
        }
    }
}
