use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        a: i64,
        b: i64,
    }

    let mut ans = 0;
    for x in -100000..=100000 {
        let v = vec![a, b, x];
        for v in v.iter().permutations(3) {
            if v[2] - v[1] == v[1] - v[0] {
                ans += 1;
                break;
            }
        }
    }
    println!("{ans}");
}
