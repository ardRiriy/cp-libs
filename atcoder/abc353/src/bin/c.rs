use cps::cumulative_sum::CumulativeSum;
use cps::veclibs::VecLibs;
use proconio::input;
fn main() {
    input!{
        n: usize,
        mut a: [u64; n],
    }
    a.sort_unstable();

    let mut ans = 0;
    let m = 1e8 as u64;
    let csum = CumulativeSum::new(&a);

    for (i, &ai) in a.iter().enumerate() {
        let l = i+1;
        let idx = a.lower_bound(m - ai);
        let r = if idx <= l { l } else { idx };

        ans += csum.get(l..r) + ai * (r - l) as u64;
        ans += csum.get(r..n) + ai * (n - r) as u64 - m * (n - r) as u64;
    }

    println!("{}", ans);
}

