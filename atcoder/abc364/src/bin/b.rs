use proconio::{input, marker::{Chars, Usize1}};
fn solve() {
    input!{
        h: usize,
        w: usize,
        mut si: Usize1,
        mut sj: Usize1,
        c: [Chars; h],
        x: Chars
    }

    let di = &[!0, 0, 1, 0];
    let dj = &[0, 1, 0, !0];
    let dc = &['U', 'R', 'D', 'L'];

    for &mv in &x {
        let idx = dc.iter().position(|&x| x == mv).unwrap();
        let ni = si.wrapping_add(di[idx]);
        let nj = sj.wrapping_add(dj[idx]);
        if ni < h && nj < w && c[ni][nj] != '#' {
            si = ni;
            sj = nj;
        }
    }
    
    println!("{} {}", si + 1, sj + 1);
}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/

static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        if *self > elm {
            *self = elm;
            true
        } else {
            false
        }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
            *self = elm;
            true
        } else {
            false
        }
    }
}

fn main() {
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}

