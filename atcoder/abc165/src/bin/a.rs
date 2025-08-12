#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        k: u64,
        a: u64,
        b: u64,
    }

    println!("{}", if (a..=b).any(|i| i%k==0) { "OK" } else { "NG" });
}
