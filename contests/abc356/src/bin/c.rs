use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let keys  = (0..m).map(|_| {
        input! {
            c: usize,
            a: [Usize1; c],
            r: char
        }
        (a, r == 'o')
    }).collect_vec();

    let mut ans = 0;

    'i: for i in 0..1<<n {
        for (v, r) in &keys {
            let num = v.iter().filter(|&&x| i >> x & 1 == 1).count();
            if !((num >= k && *r) || ( num < k && !*r)) {
                continue 'i;
            }
        }
        ans += 1;
    }

    println!("{ans}");
}
