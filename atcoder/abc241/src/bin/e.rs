use cps::cumulative_sum::CumulativeSum;
use proconio::input;


fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
    }

    let mut v = vec![];
    let mut idxs = vec![];
    let mut used = vec![false; n];
    let mut cur = 0;
    while !used[cur%n] {
        used[cur%n] = true;
        v.push(a[cur%n] as usize);
        idxs.push(cur%n);

        cur += a[cur%n] as usize;
    }

    let base = idxs.iter().position(|&val| val == cur%n).unwrap();
    let length = v.len()-base;
    let csum = CumulativeSum::new(&v);

    if k <= v.len() {
        println!("{}", csum.get(0..k));
    } else {
        let k = k - v.len();
        dbg!(base);
        dbg!(&v);
        dbg!(base+(k%length));
        let ans = csum.get(0..v.len()) + csum.get(base..v.len()) * (k/length) + csum.get(base..base+(k%length));
        println!("{}", ans);
    }
}

