use std::cmp::Reverse;

use cps::chlibs::ChLibs;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        mut b: [i64; n],
        mut w: [i64; m],
    }
    b.sort_unstable_by_key(|bi| Reverse(*bi));
    w.sort_unstable_by_key(|wi| Reverse(*wi));

    let mut cur = 0;
    let mut cur_sum = 0;
    // i番目での価値としてあり得る最大
    let mut c_max = vec![];
    for &bi in &w {
        cur_sum += bi;
        cur.chmax(cur_sum);
        c_max.push(cur);
    }

    let mut ans = 0;
    let mut w_sum = 0;
    for (i, &wi) in b.iter().enumerate() {
        w_sum += wi;
        ans.chmax(w_sum+c_max[i.min(c_max.len()-1)]);
    }

    println!("{ans}");
}

