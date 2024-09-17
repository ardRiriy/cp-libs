use ac_library::ModInt998244353;
use proconio::{input};
fn main() {
    input!{
        n: i64,
    }
    let m = ModInt998244353::new(n);
    println!("{}", m);
}
