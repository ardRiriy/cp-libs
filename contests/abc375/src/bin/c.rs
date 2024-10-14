use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        n: usize,
        a: [Chars; n],
    }

    let mut ans = vec![vec!['.'; n]; n];

    fn operated_index(n: usize, i: usize, j: usize, val: usize) -> (usize, usize) {
        let mut i = i;
        let mut j = j;

        for _ in 0..val%4 {
            let ti = i;
            i = n - j - 1;
            j = ti;
        }
        (i, j)
    }

    for i in 0..n {
        for j in 0..n {
            let val = (i+1).min(j+1).min(n-i).min(n-j);
            let (pi, pj) = operated_index(n, i, j, val);
            ans[i][j] = a[pi][pj];
        }
    }
    println!("{}", ans.iter().map(|v| v.iter().join("")).join("\n"));

}
