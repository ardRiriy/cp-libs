use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        n: usize,
        m: usize,   
    }

    let v = (0..m).fold(vec![], |mut acc, _| {
        input! {
            k: usize,
            v: [i32; k]
        }
        acc.push(v);
        acc
    });

    let ans = (1..=n).all(|i| {
        let vv = v
            .iter()
            .filter_map(|v| if v.contains(&(i as i32)) { Some(v) } else { None })
            .flatten().sorted().dedup().collect_vec();
        vv.len() == n
    });

    println!("{}", if ans { "Yes" } else { "No" });
}

