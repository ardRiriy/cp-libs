use cps::cumulative_sum::CumulativeSum;
use cps::veclibs::VecLibs;
use cps::consts::INF;
use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        m: usize,
        k: u64,
        a: [u64; n],
    }

    let rem = k - a.iter().sum::<u64>();
    if n == m {
        println!("{}", (0..n).map(|_| 0).join(" "));
        return;
    }

    let a = a.iter()
        .enumerate()
        .map(|(idx, &x)| (x, idx))
        .sorted()
        .collect_vec();

    let sorted_a = a.iter()
        .map(|(x, _)| *x)
        .collect_vec();

    let csum = CumulativeSum::new(&sorted_a);

    let mut ans = vec![INF; n];

    for (i, &(x, idx)) in a.iter().enumerate() {
        let mut ng = !0;
        let mut ok = rem + 1;

        while ok.wrapping_sub(ng) > 1 {
            // 人idxがX票獲得した時、
            // 人idxを除いた上位M人が人idxよりも上位に立つために、まだX+1票未満の人がmid+1に達するまでに必要な総票数をX'とする。
            // X + X' > K - ΣA
            // ならば、人idxより上位にM人いないので、その票数で当確である。
            let mid = ok.wrapping_add(ng) / 2;

            let rid = sorted_a.lower_bound(x + mid + 1);
            let lid = n - m - if i >= n - m { 1 } else { 0 };

            let mut cnt = 0;
            if rid > lid {
                cnt += (rid - lid) as u64 * (x + mid + 1) - csum.get(lid..rid);
            }

            if lid <= i && i < rid {
                cnt -= 1;
            } else {
                cnt += mid;
            }

            if cnt > rem {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if ng != rem {
            ans[idx] = ok;
        }
    }
    println!("{}", ans
        .iter()
        .map(|x| if *x == INF {
            -1
        } else {
            *x as i64
        })
        .join(" "));
}
