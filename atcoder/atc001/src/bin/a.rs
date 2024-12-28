use cps::consts::{DI, DJ};
use proconio::{input, marker::Chars};
fn main() {
    input!{
        h: usize,
        w: usize,
        mut c: [Chars; h],
    }

    let pos = |chr: char| -> (usize, usize) {
        let res = c.iter().flatten().position(|elm| *elm == chr).unwrap();
        (res / w, res % w)
    };

    let mut stk = vec![pos('s')];
    while let Some((pi, pj)) = stk.pop() {
        for r in 0..4 {
            let ni = pi.wrapping_add(DI[r]);
            let nj = pj.wrapping_add(DJ[r]);
            if ni < h && nj < w {
                if c[ni][nj] == 'g' {
                    println!("Yes");
                    return;
                } else if c[ni][nj] == '.' {
                    c[ni][nj] = '_';
                    stk.push((ni, nj));
                }
            }
        }
    }
    println!("No");
}
