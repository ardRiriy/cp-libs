use library::{cumulative_sum::CumulativeSum, utils::input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let v = (0..n)
        .map(|_| ip.triple::<u64, u64, u64>())
        .collect::<Vec<_>>();

    let b = v.iter().map(|&(_, _, b)| b).collect::<Vec<_>>();

    let csum = CumulativeSum::new(&b);

    static M: u64 = 1000;
    // dp[i][j] := i番目にテンションがjで始めたときの最終的な値
    let mut dp = vec![vec![0; M as usize + 1]; n + 1];
    dp[n] = (0..=M).collect::<Vec<u64>>();

    for i in (0..n).rev() {
        for j in 0..=M {
            if j <= v[i].0 {
                dp[i][j as usize] = dp[i + 1][(j + v[i].1) as usize];
            } else {
                let k = if j >= v[i].2 { j - v[i].2 } else { 0 };
                dp[i][j as usize] = dp[i + 1][k as usize];
            }
        }
    }

    let q = ip.next::<usize>();
    for _ in 0..q {
        let x = ip.next::<u64>();

        let i = if x <= 500 {
            0
        } else {
            let targ = x - 500;
            csum.binary_search(targ).unwrap_or_else(|x| x)
        };

        if i == n + 1 {
            println!("{}", x - csum.get(..));
        } else {
            let s = x - csum.get(..i);
            println!("{}", dp[i][s as usize]);
        }
    }
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip);
    }
}
