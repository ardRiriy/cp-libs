use cps::chlibs::ChLibs;
use cps::consts::INF;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        k: [u64; n],
    }
    let mut ans = INF;
    for i in 0..1<<n {
        let mut a = 0;
        let mut b = 0;

        for j in 0..n {
            if i >> j & 1 == 1 {
                a += k[j];
            } else {
                b += k[j];
            }
        }

        if a != 0 && b != 0 {
            ans.chmin(a.max(b));
        }
    }
    println!("{ans}");
}
