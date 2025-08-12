#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        x: u64,
    }

    let mut cur = 100;
    let mut ans = 0;
    while cur < x {
        cur += cur / 100;
        ans += 1;
    }
    println!("{}", ans);
}

