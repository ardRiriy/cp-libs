use library::utils::{input::Input, iterlibs::dedup::RleItertor};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let a = ip.vector::<u32>(n);
    let b = ip.vector::<u32>(n);
    let ac = a.iter().sum::<u32>();
    let bc = b.iter().sum::<u32>();
    
    if ac != bc {
        println!("No");
        return;
    }
    
    if a == b {
        println!("Yes");
        return;
    }
    
    println!("{}", if ac == 1 && (a.iter().rle().count() == 2 || b.iter().rle().count() == 2) {
        "No"
    } else {
        "Yes"
    });


}

fn main() {
    static IS_MULTI_TEST_CASE :bool = true;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip);
    }
}


// ===== bundled library =====

pub mod library {
    pub mod utils {
        pub mod input {
            use std::str::{from_utf8, FromStr};
            pub struct Input {
                buf: Vec<u8>,
                pos: usize,
            }
            impl Input {
                pub fn new() -> Self {
                    Self { buf: Vec::new(), pos: 0 }
                }
                pub fn next<T: FromStr>(&mut self) -> T {
                    while self.pos < self.buf.len()
                        && self.buf[self.pos].is_ascii_whitespace()
                    {
                        self.pos += 1;
                    }
                    let start = self.pos;
                    while self.pos < self.buf.len()
                        && !self.buf[self.pos].is_ascii_whitespace()
                    {
                        self.pos += 1;
                    }
                    if start == self.pos {
                        let mut input = String::new();
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        self.buf.clear();
                        self.buf.extend(input.as_bytes());
                        self.pos = 0;
                        return self.next();
                    }
                    from_utf8(&self.buf[start..self.pos])
                        .unwrap()
                        .parse::<T>()
                        .ok()
                        .expect(
                            &format!(
                                "Failed to parse input: {}", from_utf8(& self.buf[start
                                ..self.pos]).unwrap()
                            ),
                        )
                }
                #[allow(non_snake_case)]
                pub fn vector<T: FromStr>(&mut self, n: usize) -> Vec<T> {
                    (0..n).map(|_| self.next()).collect()
                }
                pub fn graph(
                    &mut self,
                    n: usize,
                    m: usize,
                    is_one_way: bool,
                ) -> Vec<Vec<usize>> {
                    let mut graph = vec![Vec::new(); n];
                    for _ in 0..m {
                        let (u, v): (usize, usize) = self.pair();
                        graph[u - 1].push(v - 1);
                        if !is_one_way {
                            graph[v - 1].push(u - 1);
                        }
                    }
                    graph
                }
                pub fn weighted_graph<T: Copy + FromStr>(
                    &mut self,
                    n: usize,
                    m: usize,
                    is_one_way: bool,
                    is_one_based: bool,
                ) -> Vec<Vec<(usize, T)>> {
                    let mut graph = vec![Vec::new(); n];
                    for _ in 0..m {
                        let (u, v, w): (usize, usize, T) = self.triple();
                        let u = if is_one_based { u - 1 } else { u };
                        let v = if is_one_based { v - 1 } else { v };
                        graph[u].push((v, w));
                        if !is_one_way {
                            graph[v].push((u, w));
                        }
                    }
                    graph
                }
                pub fn pair<T: FromStr, U: FromStr>(&mut self) -> (T, U) {
                    (self.next(), self.next())
                }
                pub fn triple<T: FromStr, U: FromStr, V: FromStr>(
                    &mut self,
                ) -> (T, U, V) {
                    (self.next(), self.next(), self.next())
                }
            }
        }
        pub mod iterlibs {
            pub mod dedup {
                use std::iter::Peekable;
                pub struct Rle<I: Iterator> {
                    iter: Peekable<I>,
                }
                impl<I> Iterator for Rle<I>
                where
                    I: Iterator,
                    I::Item: PartialEq,
                {
                    type Item = (usize, I::Item);
                    fn next(&mut self) -> Option<Self::Item> {
                        let mut len = 1;
                        let val = self.iter.next()?;
                        while let Some(nxt) = self.iter.peek() {
                            if &val == nxt {
                                self.iter.next()?;
                                len += 1;
                            } else {
                                break;
                            }
                        }
                        Some((len, val))
                    }
                }
                pub trait RleItertor: Iterator + Sized {
                    fn rle(self) -> Rle<Self>
                    where
                        Self::Item: PartialEq,
                    {
                        Rle { iter: self.peekable() }
                    }
                }
                impl<I: Iterator> RleItertor for I {}
                pub struct Dedup<I>
                where
                    I: Iterator,
                {
                    inner: Rle<I>,
                }
                impl<I> Iterator for Dedup<I>
                where
                    I: Iterator,
                    I::Item: PartialEq,
                {
                    type Item = I::Item;
                    fn next(&mut self) -> Option<Self::Item> {
                        self.inner.next().map(|(_, val)| val)
                    }
                }
                pub trait DedupIterator: Iterator + Sized {
                    fn dedup(self) -> Dedup<Self>
                    where
                        Self::Item: PartialEq,
                    {
                        Dedup { inner: self.rle() }
                    }
                }
                impl<I: Iterator> DedupIterator for I {}
            }
        }
    }
}

