use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        n: usize,
        s: usize,
        cards: [(usize, usize); n]
    }

    let mut dp = vec![(false, vec![]); s+1];
    let mut next = vec![(false, vec![]); s+1];

    dp[0].0 = true;

    for &(a, b) in &cards {
        next.fill((false, vec![]));
        for i in (0..s).rev() {
            if !dp[i].0 { continue; }

            if a + i <= s {
                next[a+i] = dp[i].clone();
                next[a+i].1.push('H');
            }

            if b + i <= s {
                next[b+i] = dp[i].clone();
                next[b+i].1.push('T');
            }
        }
        dp = next.clone();
        // println!("{:?}", dp);
    }

    if dp[s].0 {
        println!("Yes");
        println!("{}", dp[s].1.iter().join(""));
    } else {
        println!("No");
    }
}
