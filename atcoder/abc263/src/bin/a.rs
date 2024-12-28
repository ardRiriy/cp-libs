use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        v: [Usize1; 5],
    }

    let v = v.iter()
        .fold(vec![0; 13], |mut acc, x| {
        acc[*x] += 1;
        acc
    })
        .iter()
        .sorted()
        .rev()
        .copied()
        .collect::<Vec<i32>>();

    println!("{}",
        if v[0] == 3 && v[1] == 2 {
            "Yes"
        } else {
            "No"
        }
    );
}
