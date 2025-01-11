use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [Chars; h]
    }

    let v = ss
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(idx, c)| {
            if *c == 'o' {
                Some((idx / w, idx % w))
            } else {
                None
            }
        })
        .collect_vec();

    eprintln!("{:?}", v);
    println!("{}", v[0].0.abs_diff(v[1].0) + v[0].1.abs_diff(v[1].1));
}
