#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        mut a: [i64; n],
    }

    let mut imos = vec![0; n];
    let mut current = 0;
    for i in 0..n {
        current += imos[i];
        a[i] += current;

        let left = n - i - 1;


        let give = if a[i] > left as i64 { left as i64 } else { a[i] };
        if give != left as i64{
            imos[i+a[i] as usize+1] -= 1;
        }
        a[i] -= give;
        current += 1;
    }
    println!("{}", a.iter().join(" "));

}

