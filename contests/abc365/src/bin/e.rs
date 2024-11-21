use proconio::input;
static INF: u64 = 1 << 60;

fn main() {
    input!{
        n: usize,
        a: [u64; n]
    }

    let mut k = 1u64;
    let mut ans = 0u64;
    for i in 0..60 {
        let mut dp = vec![vec![INF; 2]; n+1];
        let mut sum = 0;

        for j in 1..=n { 
            dp[j][0] = 0;
            dp[j][1] = 0;
            dp[j][(a[j-1] >> i & 1) as usize] = 1;
            for k in 0..2 {
                if dp[j-1][k] == INF { continue; }
                let val = dp[j-1][k];
                dp[j][k ^ (a[j-1] >> i & 1) as usize] += val;
            }    
            sum += dp[j][1] - (a[j-1] >> i & 1);
        }

        // println!("{:?}", dp);
        ans += sum * k;
        // println!("{:?}", dp[n]);
        k *= 2;
    }
    println!("{}", ans);
}
