use std::io::BufRead;
use std::str::{self, FromStr};
use std::vec;

use itertools::iproduct;
use proconio::input;

fn solve() {
    // let h: usize = rdr.r();
    // let w: usize = rdr.r();
    // let k: usize = rdr.r();
    // let (si, sj): (usize, usize) = (rdr.r(), rdr.r());
    // let a: Vec<Vec<u64>> = (0..h).map(|_| rdr.rv(w)).collect();


    input! {
        h: usize,
        w: usize,
        k: usize,
        si: usize,
        sj: usize,
        a: [[u64; w]; h]
    }

    let di :&[usize] = &[0, 1, 0, !0, 0];
    let dj :&[usize] = &[1, 0, !0, 0, 0];

    let n = 50000;
    if k >= n {
        
        let mut dp = vec![vec![vec![INF; w]; h]; n+1];
        dp[0][si - 1][sj - 1] = 0;

        for i in 0..n {
            for (hi, wi) in iproduct!(0..h, 0..w) {
                if dp[i][hi][wi] == INF {
                    continue;
                }

                for r in 0..5 {
                    let ni = hi.wrapping_add(di[r]);
                    let nj = wi.wrapping_add(dj[r]);

                    if ni >= h || nj >= w {
                        continue;
                    }

                    if dp[i + 1][ni][nj] == INF {
                        dp[i + 1][ni][nj] = dp[i][hi][wi] + a[ni][nj];
                    } else {
                        let next = dp[i][hi][wi] + a[ni][nj];
                        dp[i+1][ni][nj].chmax(next);
                    }
                }
            } 
        }

        let ans = dp[n].iter().flatten().filter_map(|x| if *x == INF { None } else { Some(*x) }).max().unwrap();
        println!("{}", ans + (k - n) as u64 * a.iter().flatten().max().unwrap());

        return;
    }

    let mut dp = vec![vec![vec![INF; w]; h]; k + 1];
    dp[0][si - 1][sj - 1] = 0;

    for i in 0..k {
        for (hi, wi) in iproduct!(0..h, 0..w) {
            if dp[i][hi][wi] == INF {
                continue;
            }

            for r in 0..5 {
                let ni = hi.wrapping_add(di[r]);
                let nj = wi.wrapping_add(dj[r]);

                if ni >= h || nj >= w {
                    continue;
                }


                if dp[i + 1][ni][nj] == INF {
                    dp[i + 1][ni][nj] = dp[i][hi][wi] + a[ni][nj];
                } else {
                    let next = dp[i][hi][wi] + a[ni][nj];
                    dp[i+1][ni][nj].chmax(next);
                }
            }
        } 
    }

    let ans = dp[k].iter().flatten().filter_map(|x| if *x == INF { None } else { Some(*x) }).max().unwrap();
    println!("{}", ans);

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

pub struct StdinReader<R: BufRead> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}

impl<R: BufRead> StdinReader<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf: Vec::new(),
            pos: 0,
        }
    }

    fn r<T: FromStr>(&mut self) -> T {
        self._read_until(&[b' ', b'\n'])
    }

    fn rv<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        let mut v: Vec<T> = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.r());
        }
        v
    }

    fn _read_until<T: FromStr>(&mut self, delims: &[u8]) -> T {
        loop {
            if self.pos < self.buf.len() {
                let start = self.pos;
                while self.pos < self.buf.len() && !delims.contains(&self.buf[self.pos]) {
                    self.pos += 1;
                }

                if self.pos < self.buf.len() {
                    let end = self.pos;
                    self.pos += 1;
                    let token: &[u8] = &self.buf[start..end];
                    let elem: &str = unsafe { str::from_utf8_unchecked(token) };

                    return elem
                        .parse()
                        .unwrap_or_else(|_| panic!("{}", format!("failed parsing: {}", elem)));
                }
            }

            self.buf.clear();
            self.pos = 0;
            let len = self
                .reader
                .read_until(b'\n', &mut self.buf)
                .expect("failed reading bytes");
            if len == 0 {
                panic!("early eof");
            }
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

