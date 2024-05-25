use proconio::input;

fn solve() {
    input! {
        n: usize,
        d: usize,
        mut w: [u64; n]
    }

    let mut dp = vec![vec![1e18; d + 1]; 1 << n];
    let avg = w.iter().sum::<u64>() as f64 / d as f64;

    // init
    for i in 0..1 << n {
        let sum = w
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| if i & 1 << idx != 0 { Some(*x) } else { None })
            .sum::<u64>();
        dp[i][1] = (sum as f64 - avg).powf(2.0);
    }

    // update
    for s in 0..1 << n {
        for k in 2..=d {
            dp[s][k] = dp[s][k - 1] + dp[0][1];
            let mut t = s;
            while t > 0 {
                let diff = s ^ t;
                let val = dp[diff][k - 1] + dp[t][1];
                if dp[s][k] > val {
                    dp[s][k] = val;
                }
                t = (t - 1) & s;
            }
        }
    }

    println!("{}", dp[(1 << n) - 1][d] / d as f64);
}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/

static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
    }
}

fn main() {
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
