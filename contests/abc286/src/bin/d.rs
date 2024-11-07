use proconio::input;
fn main() {
    input!{
        n: usize,
        x: usize,
        coins: [(usize, u64); n],
    }
    let mut dp = vec![false; x+1];
    dp[0] = true;
    
    for (ai, k) in coins {
        for _ in 0..k {
            for i in (0..x).rev() {
                if i + ai <= x && dp[i] {
                    dp[i + ai] = true;
                }
            }
        }
    }
    println!("{}", if dp[x] { "Yes" } else { "No" });
}

