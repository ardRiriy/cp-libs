use proconio::{input, marker::Chars};

fn main() {
    input!{
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        mut s: Chars,
    }
    s.insert(0, '0');

    let mut ans = vec![false; 1<<n];

    for i in 0..n {
        if s[1<<i] == '0' {
            ans[1<<i] = true;
        }
    }

    for i in 1..(1<<n) {
        if !ans[i] { continue; }

        for j in 0..n {
            let nxt = i | (1 << j);
            if s[nxt] == '0' {
                ans[nxt] = true;
            }
        }
    }

    println!("{}", if ans[(1<<n)-1] { "Yes" } else { "No" });
}