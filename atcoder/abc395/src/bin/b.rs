use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input!{
        n: usize,
    }
    let mut res = vec![vec!['.'; n]; n];
    for (i, j) in iproduct!(0..n, 0..n) {
        let di = i.min(n-i-1);
        let dj = j.min(n-j-1);
        
        if di.min(dj) % 2 == 0 {
            res[i][j] = '#';
        }
    }
    println!("{}", res.iter().map(|v| v.iter().join("")).join("\n"));
}

