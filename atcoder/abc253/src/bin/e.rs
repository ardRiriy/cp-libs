use std::io::BufRead;
use std::str::{self, FromStr};
use ac_library::ModInt998244353;

fn solve(rdr: &mut StdinReader<impl BufRead>) {
    let n: usize = rdr.r();
    let m: usize = rdr.r();
    let k: i64 = rdr.r();

    let mut dp = vec![vec![ModInt998244353::new(0); m]; n];
    let mut sum = vec![ModInt998244353::new(0); m+1];
    for i in 0..m {
        dp[0][i] = ModInt998244353::new(1);
        sum[i + 1] = ModInt998244353::new(1) + sum[i];    
    }


    for i in 1..n {
        eprintln!("sum: {:?}", sum);

        let mut v = vec![ModInt998244353::new(0); m+1];
        for j in 0..m {
            dp[i][j] += sum[((j + 1) as i64 - k).max(0) as usize] - sum[0];
            dp[i][j] += sum[m] - sum[((j + 1) as i64 + k - 1) as usize]; 
            v[j+1] = v[j] + dp[i][j];
        }
        sum = v.clone();
        eprintln!("dp[{}]: {:?}",i ,dp[i]);
    }

    println!("{}", dp[n-1].iter().sum::<ModInt998244353>());


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
