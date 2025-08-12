use ac_library::ModInt998244353 as Mint;
use proconio::input;

fn sum(n: Mint) -> Mint {
    n * (n+1) / 2
}

fn main() {
    input!{
        n: u64,
    }
    let mut ans = Mint::new(0);
    let mut b = 2;

    while b <= n {
        let zc = n/b;

        let mut ok = b;
        let mut ng = n+1;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok+ng)/2;
            if n/mid == zc {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        // [b, ok] までの個数を数え上げ
        ans += Mint::new(ok-b+1) * (n + 1 - zc);
        ans -= sum(Mint::new(ok)) - sum(Mint::new(b-1));

        b = ok+1;
    }
    println!("{}", ans);
}

// 1000000000000