use proconio::{input};
fn main() {
    input!{
        n: usize,
        a: [i64; n]
    }

    let mut ans = n;
    let mut l = 0;
    let mut r = l+1;

    let sum = |x| x * (x+1) / 2;
    while l < n && r < n {
        let diff = a[r] - a[l];
        while r + 1 < n && a[r+1] - a[r] == diff {
            r += 1;
        }
        ans += sum(r - l);
        // dbg!((sum(r - l), r, l));
        l = r;
        r = l + 1; 
    }

    println!("{ans}")
}

