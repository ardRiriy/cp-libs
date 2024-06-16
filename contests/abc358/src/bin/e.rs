use std::collections::BTreeMap;
use std::io::BufRead;
use std::str::{self, FromStr};

use ac_library::{Mod998244353, ModInt998244353, StaticModInt};
use proconio::input;

struct Loop {
    memo: BTreeMap<Vec<u64>, StaticModInt<Mod998244353>>,
    max: Vec<u64>,
    max_dpt: usize
}

impl Loop {
    fn new(k: usize, c: &Vec<u64>) -> Self {
        Loop {
            memo: BTreeMap::new(),
            max: c.clone(),
            max_dpt: k
        }
    }

    fn exec(&mut self, cnt: &mut Vec<u64>, dpt: usize) -> StaticModInt<Mod998244353> {
        if let Some(&res) = self.memo.get(cnt) {
            return res;
        }

        if dpt == self.max_dpt + 1 {
            return ModInt998244353::new(0);
        }

        let mut res = ModInt998244353::new(0);
        if dpt != 0 {
            // 空文字列でなければカウント
            res += 1;
        }

        for i in 0..26 {
            if cnt[i] < self.max[i] {
                cnt[i] += 1;
                res += self.exec(cnt, dpt + 1);
                cnt[i] -= 1;
            }
        }

        self.memo.insert(cnt.clone(), res);

        res
    } 
}

fn solve() {

    input! {
        k: usize,
        c: [u64; 26]
    }

    let mut memo = Loop::new(k, &c);
    let ans = memo.exec(&mut vec![0; 26], 0);

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
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
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

