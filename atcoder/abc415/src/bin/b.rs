use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        s: Chars
    }

    let v = s.iter()
        .enumerate()
        .filter_map(|(i, &c)| if c=='#' { Some(i+1) } else { None })
        .collect_vec();

    for i in 0..v.len()/2 {
        println!("{},{}", v[i*2], v[i*2+1]);
    }

}

