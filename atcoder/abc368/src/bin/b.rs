use proconio::{input};
fn main() {
    input!{
        n: usize,
        mut a: [u64; n]
    }

    let mut ans = 0;
    loop {
        a.sort();
        a.reverse();
        if a[0] == 0 || a[1] == 0 {
            break;
        }

        a[0] -= 1;
        a[1] -= 1;
        ans += 1;
    }

    println!("{}", ans);
}
