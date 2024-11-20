use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [Chars; n],
    }

    for i in 1..=n {
        for v in (0..n).combinations(i) {
            let mut seen = vec![false; m];
            for &idx in &v {
                for (jdx, &c) in ss[idx].iter().enumerate() {
                    if c == 'o' {
                        seen[jdx] = true;
                    }
                }
            }
            
            if seen.iter().all(|&x| x) {
                println!("{}", i);
                return;
            }
        } 
    }
}
