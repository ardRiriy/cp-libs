use std::iter::repeat;

use library::utils::{consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair();
    let w = ip.vector::<u64>(n);
    let g = ip.graph(n, m, false);

    let mut dist = vec![vec![INF; n]; n];
    dist[0] = repeat(0).take(n).collect();
    for k in (1..n).rev() {
        for p in 0..n {
            if dist[p][k] == INF { continue; }
            let cost = w[p] * k as u64;
            for &ni in g[p].iter() {
                dist[ni][k-1] = dist[ni][k-1].min(dist[p][k] + cost);
            }
        }
    }
    
    println!("{}", dist.iter().map(|v| v[0].to_string()).collect::<Vec<_>>().join("\n"));
}

fn main() {
    static IS_MULTI_TEST_CASE :bool = false;
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
        pub mod consts {
            pub static INF: u64 = 1e18 as u64;
            pub static DI: &[usize] = &[0, !0, 0, 1, !0, 1, !0, 1];
            pub static DJ: &[usize] = &[!0, 0, 1, 0, !0, !0, 1, 1];
            pub static DC: &[char] = &['L', 'U', 'R', 'D'];
        }
        pub mod input {
            use std::str::{from_utf8, FromStr};
            pub struct Input {
                buf: Vec<u8>,
                pos: usize,
            }
            impl Default for Input {
                fn default() -> Self {
                    Self::new()
                }
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
                        .unwrap_or_else(|| {
                            panic!(
                                "Failed to parse input: {}", from_utf8(& self.buf[start
                                ..self.pos]).unwrap()
                            )
                        })
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
                        let (u, v) = self.pair::<usize>();
                        let w = self.next::<T>();
                        let u = if is_one_based { u - 1 } else { u };
                        let v = if is_one_based { v - 1 } else { v };
                        graph[u].push((v, w));
                        if !is_one_way {
                            graph[v].push((u, w));
                        }
                    }
                    graph
                }
                pub fn pair<T: FromStr>(&mut self) -> (T, T) {
                    (self.next(), self.next())
                }
                pub fn triple<T: FromStr>(&mut self) -> (T, T, T) {
                    (self.next(), self.next(), self.next())
                }
                pub fn chars(&mut self) -> Vec<char> {
                    self.next::<String>().chars().collect::<Vec<_>>()
                }
            }
        }
    }
}

