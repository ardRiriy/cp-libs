use std::io::BufRead;
use std::str::{self, FromStr};

fn solve(rdr: &mut StdinReader<impl BufRead>) {
    let a: u64 = rdr.r();
    let b: u64 = rdr.r();
    let c: u64 = rdr.r();
    let d: u64 = rdr.r();

    let primes: &[u64] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199,
    ];
    't: for t_choose in a..=b {
        for a_choose in c..=d {
            if primes.contains(&(t_choose + a_choose)) {
                continue 't;
            }
        }
        println!("Takahashi");
        return;
    }
    println!("Aoki");
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
    let mut reader = StdinReader::new(std::io::stdin().lock());
    while i != 0 {
        solve(&mut reader);
        i -= 1;
    }
}
