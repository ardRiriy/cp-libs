use std::collections::BTreeSet;

use proconio::input;
fn main() {
    input!{
        n: usize,
        m: usize,
        q: [(i64, i64); m]
    }

    let mut ans = BTreeSet::new();
    let di = vec![2, 1, -1, -2, -2, -1, 1, 2];
    let dj  = vec![1, 2,2,1,-1,-2,-2,-1];

    for (i, j) in q {
        ans.insert((i, j));
        for r in 0..di.len() {
            let pi = i + di[r];
            let pj = j + dj[r];
            if pi >= 1 && pj >= 1 && pi <= n as i64 && pj <= n as i64 {
                ans.insert((pi, pj));
            } 
        }
    }
    println!("{}", n * n - ans.len());
}

