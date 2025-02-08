use itertools::Itertools;
use proconio::input;

fn main() {
    let n = 3;
    input! {
        a: [i32; n],
    }
    println!(
        "{}",
        if a.iter()
            .copied()
            .permutations(n)
            .any(|v| v[0] * v[1] == v[2])
        {
            "Yes"
        } else {
            "No"
        }
    );
}
