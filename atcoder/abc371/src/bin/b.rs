use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        a: [(Usize1, char); m]
    }

    let mut checked = vec![false ;n];
    let mut ans = vec![];
    for &(i, c) in &a {
        if checked[i] || c == 'F' {
            ans.push("No");
        } else {
            checked[i] = true;
            ans.push("Yes");
        }
    }
    println!("{}", ans.iter().join("\n"));
}
