use proconio::input;

// dp[i][j] := マスi, jに到達したときにあり得る残り枚数の最大値
// dp[i+1][j] <- dp[i][j] + a[i][j] - p[i+j]
// dp[i][j+1] <- dp[i][j] + a[i][j] - p[i+j]
fn main() {
    input!{
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        p: [i64; h+w-1],
    }

    let dp_x = |x: i64| -> bool {
        // 初期の枚数をxとしたときに、h+w-1回の操作で一度も負にならない
        let mut dp = vec![vec![-1; w]; h];
        dp[0][0] = x + a[0][0];

        for i in 0..h {
            for j in 0..w {
                if dp[i][j] < 0 {
                    continue;
                }

                // 下に移動
                if i + 1 < h {
                    let next = dp[i][j] - p[i+j];
                    if next >= 0 {
                        dp[i + 1][j] = dp[i + 1][j].max(next + a[i + 1][j]);
                    }
                }

                // 右に移動
                if j + 1 < w {
                    let next = dp[i][j] - p[i+j];
                    if next >= 0 {
                        dp[i][j + 1] = dp[i][j + 1].max(next + a[i][j + 1]);
                    }
                }
            }
        }

        dp[h-1][w-1] - p[h+w-2] >= 0
    };

    
    let mut ok = p.iter().sum::<i64>()+1;
    let mut ng = -1;
    while (ok-ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if dp_x(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

