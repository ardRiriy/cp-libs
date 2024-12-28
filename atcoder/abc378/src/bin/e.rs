use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: i64,
        a: [i64; n],
    }

    let mut bit = FenwickTree::new(n+1, 0i64);
    for i in 0..n+1 {
        bit.add(i, 1);
    }
    
    let mut csum = vec![0];
    let mut sub_sum = 0;

    for (idx, &ai) in a.iter().enumerate() {
        csum.push((csum[idx] + ai) % m);
        sub_sum += csum[idx+1];
    }


    let compressed_idx = csum.iter()
        .enumerate()
        .map(|(idx, &x)| (x, idx))
        .sorted() // v[i] := idxの値がi番目
        .enumerate()
        .fold(vec![0; n+1], |mut v, (i, (_, idx))| {
            v[idx] = i;
            v
        });
    let mut ans = 0;
    for i in 0..n {
        ans += sub_sum + m * bit.sum(..compressed_idx[i]) - csum[i] * (n - i) as i64;
        sub_sum -= csum[i+1];
        bit.add(compressed_idx[i], -1);
    }

    println!("{ans}");

}
