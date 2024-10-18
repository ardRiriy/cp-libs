use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let inf = 1u64 << 60;
    let mut v = vec![(inf, 0)];
    let mut ans_v = vec![];
    let mut sum_t = 0;

    for &x in a.iter() {
        let mut len = 1;
        let mut ans = x;
        while let Some((lh, ll)) = v.pop() {
            if lh <= x {
                len += ll;
                ans += (x - lh) * ll;
            } else {
                v.push((lh, ll));
                break;
            }
        }
        ans_v.push(sum_t + ans + 1);
        sum_t += ans;
        v.push((x, len));
    }

    println!("{}", ans_v.iter().join(" "));
}

