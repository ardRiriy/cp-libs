use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        x: [i64; n],
    }

    let c = (0..m).fold(vec![0; n+1], |mut v, _| {
            input! {
                c: usize,
                y: i64,
            }
            v[c] = y;
            v
        });
    
    let inf = -1;
    let mut dp = vec![vec![inf; n+1]; n+1];
    dp[0][0] = 0;
    
    for i in 0..n {
        for j in 0..n {
            if dp[i][j] == inf {
                continue;
            }
            dp[i+1][0] = dp[i+1][0].max(dp[i][j]);
            dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j] + x[i] + c[j+1]);
        }
    }
    
    println!("{}", dp[n].iter().max().unwrap());
}
