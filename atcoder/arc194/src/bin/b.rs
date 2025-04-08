use ac_library::FenwickTree;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        p: [Usize1; n],
    }

    let mut bit = FenwickTree::new(n, 0i64);
    let mut pos = vec![0; n];
    for i in 0..n {
        pos[p[i]] = i;
    }

    fn sum(n: usize) -> usize {
        // 1からnまでの和
        n * (n + 1) / 2
    }

    let mut ans = 0;
    for i in (0..n).rev() {
        let correct_pos = bit.sum(0..pos[i]);
        bit.add(pos[i], 1);

        let val = sum(i) - sum(pos[i]-correct_pos as usize);
        ans += val;
    }

    println!("{}", ans);
}

