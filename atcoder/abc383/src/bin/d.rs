#[allow(unused_imports)]
use cps::debug::*;
use cps::prime::create_primes;
use num_integer::Roots;
use proconio::input;

fn main() {
    input!{
        n: usize
    }

    let half_n = n.sqrt();
    let v = create_primes(half_n+1);
    let mut ans :usize = 0;
    let n = n as u64;

    for &vi in v.iter() {
        let val = vi.pow(8);
        if val > n { break; }
        ans += 1;
    }

    for i in 0..v.len() {
        let p = v[i].pow(2);
        let max_q = n / p;
        // v[j]^2 がmax_q以下である最大
        // -> v[j]^2がmax_q+1以上である最小
        let idx = v.binary_search_by_key(&(&max_q+1), |vi| vi.pow(2)).unwrap_or_else(|x|x);

        if i < idx {
            ans += idx - i - 1;
        } else {
            break;
        }
    }
    println!("{ans}");
}

