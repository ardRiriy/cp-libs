use ac_library::ModInt as Mint;
#[allow(unused_imports)]
use cps::debug::*;
#[allow(unused_imports)]
use cps::lg;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        s: Chars,
    }

    Mint::set_modulus(2019);
    let mut v = vec![0u64; 2019];
    v[0] += 1;
    let mut cs = Mint::new(0);
    let mut base = Mint::new(1);

    for &ci in s.iter().rev() {
        cs += base * ci.to_digit(10).unwrap();
        base *= 10;
        v[cs.val() as usize] += 1;
    }

    fn sum(n: u64) -> u64 {
        if n > 1 {
            n * (n - 1) / 2
        } else {
            0
        }
    }

    let ans = v.iter()
        .map(|n| sum(*n))
        .sum::<u64>();

    println!("{ans}");
}

