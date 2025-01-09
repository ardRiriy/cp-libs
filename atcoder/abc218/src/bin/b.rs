use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input! {
        p: [Usize1; 26]
    }

    let base = 'a' as usize;
    let ans = p.iter().map(|&x| (base + x) as u8 as char).collect_vec();

    println!("{}", ans.iter().join(""));
}
