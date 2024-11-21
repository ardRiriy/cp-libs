use proconio::input;
use ac_library::ModInt998244353 as Mint;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }

    let mut ans = Mint::new(0);
    let mut sum = a.iter()
        .map(|x| Mint::new(*x))
        .sum::<Mint>();


    // 下の桁を固定したほうが楽
    for j in (0..n).rev() {
        sum -= a[j];
        let val = Mint::new(a[j]) * j + sum * Mint::new(10).pow(a[j].to_string().len() as u64);
        ans += val;
    }

    println!("{ans}");
}

