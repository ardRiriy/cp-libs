use ac_library::ModInt998244353;
use proconio::input;
fn main() {
    input!{
        n: u64
    }

    let mut ans = ModInt998244353::new(0);
    let mut k = 10;

    let sum = |i| { i * (i + 1) / 2};

    for _ in 0.. {
        let upper = ModInt998244353::new((k - 1).min(n));
        // 1 から (upper - (10 ^ i - 1))の和を足す
        ans += sum(upper - (k / 10 - 1));

        if k > n {
            break;
        }
        k *= 10;
    }
    println!("{}", ans);
}
