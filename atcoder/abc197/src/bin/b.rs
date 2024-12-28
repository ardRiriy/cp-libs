use cps::consts::{DI, DJ};
use proconio::{input, marker::{Chars, Usize1}};
fn main() {
    input!{
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        a: [Chars; h],
    }

    let mut ans = 0;
    for r in 0..4 {
        let mut ni = x;
        let mut nj = y;
        while ni < h && nj < w && a[ni][nj] == '.' {
            ans += 1;
            ni = ni.wrapping_add(DI[r]);
            nj = nj.wrapping_add(DJ[r]);
        }
    }
    println!("{}", ans - 3);
}
