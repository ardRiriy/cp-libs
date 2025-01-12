use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        a: [Usize1; n],
    }
    static N: usize = 2e5 as usize;
    let mut cnt = vec![0; N];
    for &ai in a.iter() {
        cnt[ai] += 1;
    }

    let mut seen = vec![0; N]; // left
    let mut ans = 0;
    let mut dup = 0;

    for (i, ai) in a.iter().enumerate() {
        dup -= seen[*ai]*cnt[*ai];

        cnt[*ai] -= 1;

        let x = (i-seen[*ai]) * (n-i-1-cnt[*ai]);
        ans += x - dup;
        seen[*ai] += 1;
        dup += seen[*ai]*cnt[*ai];
    }
    println!("{}", ans);
}
