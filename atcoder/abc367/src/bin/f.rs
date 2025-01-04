use cps::zobrist_hash::ZobristHash;
use proconio::input;
fn main() {
    input!{
        n: usize,
        q: usize,
        a: [u64; n],
        b: [u64; n],
    }
    let mut zh = ZobristHash::new();

    let mut a_sum = vec![0];
    for (i, &ai) in a.iter().enumerate() {
        let hash = zh.get(ai);
        let val = a_sum[i] + hash;
        a_sum.push(val);
    }

    let mut b_sum = vec![0];
    for (i, &bi) in b.iter().enumerate() {
        let hash = zh.get(bi);
        let val = b_sum[i] + hash;
        b_sum.push(val);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            s: usize,
            t: usize,
        }
        let x = a_sum[r] - a_sum[l-1];
        let y = b_sum[t] - b_sum[s-1];
        println!("{}", if x == y && r - l == t - s { "Yes" } else { "No" });
    }
}
