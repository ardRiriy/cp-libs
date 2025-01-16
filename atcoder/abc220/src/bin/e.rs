use proconio::input;
use ac_library::ModInt998244353 as Mint;
fn main() {
    input!{
        n: u64,
        d: u64
    }

    let mut ans = Mint::new(0);

    fn calc(d: u64) -> Mint {
        if d == 0 {
            Mint::new(1)
        } else {
            Mint::new(2).pow(d-1)
        }
    } 

    for l in 0..=d {
        let r = d - l;
        if l >= n || r >= n {
            continue;
        }
        let l_val = calc(l);
        let r_val = calc(r);
        ans += l_val * r_val * 2;
    }
    println!("{}", ans);

}

