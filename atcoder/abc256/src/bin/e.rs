use ac_library::Dsu;
use cps::chlibs::ChLibs;
use cps::consts::INF;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        x: [Usize1; n],
        c: [u64; n],
    }

    let mut dsu = Dsu::new(n);
    let mut ans = 0;
    for i in 0..n {
        if !dsu.same(i, x[i]) {
            dsu.merge(i, x[i]);
            continue;
        }

        let mut min = INF;
        let mut current = i;
        loop {
            min.chmin(c[current]);
            current = x[current];
            if current == i {
                break;
            }
        }
        ans += min;
    }
    println!("{}", ans);
}
