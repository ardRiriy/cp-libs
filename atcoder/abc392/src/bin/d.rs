#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    static N: usize = 1e5 as usize + 2;
    let mut v = vec![];
    let mut sums = vec![];

    for _i in 0..n {
        input! {
            k: usize,
            a: [u64; k],
        }
        let a = a
            .iter()
            .copied()
            .sorted_unstable()
            .dedup_with_count()
            .collect_vec();
        sums.push(k as f64);
        v.push(a);
    }

    let mut ans: f64 = 0.;
    for i in 0..n {
        for j in i + 1..n {
            let mut ii = 0;
            let mut ji = 0;
            let mut sum = 0.;
            while ii < v[i].len() && ji < v[j].len() {
                if v[i][ii].1 == v[j][ji].1 {
                    sum += v[i][ii].0 as f64 / sums[i] * v[j][ji].0 as f64 / sums[j];
                    ii += 1;
                    ji += 1;
                } else if v[i][ii].1 < v[j][ji].1 {
                    ii += 1;
                } else {
                    ji += 1;
                }
            }
            ans = ans.max(sum);
        }
    }
    println!("{}", ans);
}
