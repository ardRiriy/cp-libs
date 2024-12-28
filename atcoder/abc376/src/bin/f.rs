use proconio::input;
fn main() {
    // solve できてない
    input!{
        n: usize,
        q: usize,
    }

    let inf = 1u64 << 60;

    // dp[i][j] := i回目のクエリ処理後、(最小値, 左手の位置, 右手の位置)
    let mut dp = vec![vec![(inf, inf, inf); 2]; q+1];
    dp[0][0] = (0, 0, 1);

    for i in 0..q {
        input! {
            h: char,
            t: u64,
        }
        let t = t - 1;

        for j in 0..=1 {
            if dp[i][j].0 == inf {
                continue;
            }
            let l = dp[i][j].1;
            let r = dp[i][j].2;
            match h {
                'L' => {
                    // 数字が大きくなる方向周り
                    if l == t {
                        if dp[i+1][0].0 > dp[i][j].0 {
                            dp[i+1][0] = dp[i][j];
                        }
                    } else if (l < r && r <= t) 
                        || (t <= r && r < l)
                        || (r < l && l < t)
                    {
                        // Rが巻き込まれる
                        let new_r = (t+1)%n as u64;
                        let val = dp[i][j].0 + (t + n as u64 - l) % n as u64 + (new_r + n as u64 - r) % n as u64;
                        if dp[i+1][0].0 > val {
                            dp[i+1][0] = (val, t, new_r);
                        }
                    } else {
                        // lだけ動かす
                        let val = dp[i][j].0 + (t + n as u64 - l) % n as u64;
                        if dp[i+1][0].0 > val {
                            dp[i+1][0] = (val, t, dp[i][j].2);
                        }
                    }

                    if l == t {
                        if dp[i+1][1].0 > dp[i][j].0 {
                            dp[i+1][1] = dp[i][j];
                        } else if (r <= t && t < l)
                        || (t < l && l < r)
                        || (l < r && r <= t)
                        {
                            let new_r = (t+n as u64-1)%n as u64;
                            let val = dp[i][j].0 + (l + n as u64 - t) % n as u64 + (r + n as u64 - new_r) % n as u64;
                            if dp[i+1][1].0 > val {
                                dp[i+1][1] = (val, t, new_r);
                            }
                        } else {
                            let val = dp[i][j].0 + (l + n as u64 - t) % n as u64;
                            if dp[i+1][1].0 > val {
                                dp[i+1][1] = (val, t, dp[i][j].2);
                            }
                        }
                    }
                }
                'R' => {
                    let tmp = l;
                    let l = r;
                    let r = tmp;
                    if l == t {
                        if dp[i+1][0].0 > dp[i][j].0 {
                            dp[i+1][0] = dp[i][j];
                        }
                    } else if (l < r && r <= t) 
                        || (t <= r && r < l)
                        || (r < l && l < t)
                    {
                        // Rが巻き込まれる
                        let new_r = (t+1)%n as u64;
                        let val = dp[i][j].0 + (t + n as u64 - l) % n as u64 + (new_r + n as u64 - r) % n as u64;
                        if dp[i+1][0].0 > val {
                            dp[i+1][0] = (val, new_r, t);
                        }
                    } else {
                        // lだけ動かす
                        let val = dp[i][j].0 + (t + n as u64 - l) % n as u64;
                        if dp[i+1][0].0 > val {
                            dp[i+1][0] = (val, dp[i][j].2, t);
                        }
                    }

                    if l == t {
                        if dp[i+1][1].0 > dp[i][j].0 {
                            dp[i+1][1] = dp[i][j];
                        } else if (r <= t && t < l)
                        || (t < l && l < r)
                        || (l < r && r <= t)
                        {
                            let new_r = (t+n as u64-1)%n as u64;
                            let val = dp[i][j].0 + (l + n as u64 - t) % n as u64 + (r + n as u64 - new_r) % n as u64;
                            if dp[i+1][1].0 > val {
                                dp[i+1][1] = (val,new_r, t);
                            }
                        } else {
                            let val = dp[i][j].0 + (l + n as u64 - t) % n as u64;
                            if dp[i+1][1].0 > val {
                                dp[i+1][1] = (val,dp[i][j].2,t);
                            }
                        }
                    }
                }
                _ =>{ unreachable!(); }
            }
        }
    }
    println!("{}", dp[q][0].0.min(dp[q][1].0));
}

