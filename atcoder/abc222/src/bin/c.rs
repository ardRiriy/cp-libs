use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        n: usize,
        m: usize,
        a: [Chars; 2*n]
    }

    let mut win_count = vec![0; 2*n];
    for i in 0..m {
        let placement = win_count.iter()
            .enumerate()
            .map(|(idx, x)| ((Reverse(*x), idx)))
            .sorted()
            .collect_vec();

        for idx in 0..2*n {
            if idx % 2 == 1 { continue; }
            // idxとidx+1
            let win = janken(a[placement[idx].1][i], a[placement[idx+1].1][i]);
            match win {
                0 => { win_count[placement[idx].1] += 1; }
                1 => { win_count[placement[idx+1].1] += 1; },
                _ => { }
            };
        }

        
    }

    let ans = win_count.iter()
        .enumerate()
        .map(|(idx, x)| (Reverse(*x), idx))
        .sorted()
        .collect_vec();

    println!("{}", ans.iter().map(|(_, idx)| idx+1).join("\n"));

}

//0 -> a win / 1 -> b win / 2 -> draw
fn janken(a: char, b: char) -> i32 {
    let chg = |c| {
        match c {
            'G' => 0,
            'C' => 1,
            'P' => 2,
            _ => unreachable!()
        }
    };

    let ai = chg(a);
    let bi = chg(b);

    let h = [vec![2, 0, 1],
        vec![1, 2, 0],
        vec![0, 1, 2]];

    h[ai][bi]
}
