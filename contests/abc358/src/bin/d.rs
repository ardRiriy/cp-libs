use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; n]
    }

    a.sort();
    b.sort();

    let mut ai = 0;

    let mut ans = 0;
    
    for bi in 0..m {
        while ai < n && b[bi] > a[ai] {
            ai += 1;
        }

        if ai == n {
            println!("-1");
            return;
        }

        ans += a[ai];
        ai += 1;
    }
    println!("{}", ans);
}
