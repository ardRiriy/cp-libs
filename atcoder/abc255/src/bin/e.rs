use std::collections::HashMap;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        s: [i64; n-1],
        x: [i64; m],
    }

    let mut dp = HashMap::new();
    // i=0の処理
    for xi in x.iter() {
        dp.insert(*xi, 1u32);
    }

    let mut sums = vec![0; 2];
    for i in 1..n {
        sums[i%2] += s[i-1];
        sums[1-i%2] -= s[i-1];

        for &xi in x.iter() {
            let a0 = (xi-sums[i%2])* if i%2==0 { 1 } else { -1 };
            if let Some(val) = dp.get_mut(&a0) {
                *val += 1;
            } else {
                dp.insert(a0, 1);
            }
        }
    }
 
    println!("{}", dp.values().max().unwrap());
}

