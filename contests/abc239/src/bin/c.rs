use itertools::Itertools;
use std::io::BufRead;
use std::str::{self, FromStr};

fn solve(rdr: &mut StdinReader<impl BufRead>) {
    let pos1: (i64, i64) = (rdr.r(), rdr.r());
    let pos2: (i64, i64) = (rdr.r(), rdr.r());

    eprintln!("{:?}", pos1);
    eprintln!("{:?}", pos2);
    let adder: &[(i64, i64)] = &[
        (1, 2),
        (2, 1),
        (-1, 2),
        (-2, 1),
        (1, -2),
        (2, -1),
        (-1, -2),
        (-2, -1),
    ];

    if adder
        .iter()
        .flat_map(|p| {
            adder
                .iter()
                .map(|pp| (pp.0 + p.0 + pos1.0, pp.1 + p.1 + pos1.1))
                .collect_vec()
        })
        .collect_vec()
        .contains(&pos2)
    {
        println!("Yes");
    } else {
        println!("No");
    }
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
