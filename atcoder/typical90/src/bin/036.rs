use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [(i64, i64); n],
    }

    let p = p.iter()
        .map(|&(x, y)| (x + y, x - y))
        .collect_vec();

    let x = p.iter()
        .map(|&(x, _)| x) 
        .sorted()
        .collect_vec();

    let y = p.iter()
        .map(|&(_, y)| y) 
        .sorted()
        .collect_vec();

    for _ in 0..q {
        input! {
            q: Usize1,
        }
        let (i, j) = p[q];

        println!("{}", x[0].abs_diff(i)
                        .max(x[n-1].abs_diff(i)) 
                        .max(y[0].abs_diff(j))
                        .max(y[n-1].abs_diff(j))
                    );
    }
}
