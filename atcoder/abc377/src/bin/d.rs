use cps::chlibs::ChLibs;
use proconio::{input, marker::Usize1};

/*
区間の左端を固定(これをLとする)した場合、そこから伸ばせる右端は
L <= l_iであるようなiのうち、r_iが最小のもの
までである。(これは ある[L, R]が[l_i, r_i]と完全にかぶらない <=> l_i > L または r_i < Rから。)

これである場所から伸ばせる右端がわかったので、答えが求まる。
*/

fn main() {
    input! {
        n: usize,
        m: usize,
        r: [(Usize1, Usize1); n]
    }

    let mut v = vec![m; m];
    for (l, r) in &r {
        v[*l].chmin(*r);
    }

    let mut min = 1 << 60;
    for i in (0..m).rev() {
        min.chmin(v[i]);
        v[i] = min;
    }

    let ans = v.iter()
        .enumerate()
        .map(|(idx, &x)| x - idx)
        .sum::<usize>();
    println!("{ans}");
}
