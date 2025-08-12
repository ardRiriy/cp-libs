use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap}};

use library::utils::{consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let (n, m, k) = ip.triple::<usize, usize, usize>();
    let a = ip.vector::<usize>(k).iter().map(|i| *i-1).collect::<Vec<_>>();
    let g = ip.weighted_graph::<u64>(n, m, false, true);
    
    let mut livehouse = BTreeSet::from_iter(a.iter().copied());
    livehouse.remove(&0);
    livehouse.remove(&(n-1));

    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0, 0, !0));

    // seen[i][j][k] := 都市i到達時点で(都市0, n-1を除いて)j回ライブをしており、その時の移動距離の小さい方からk番目
    let mut seen = vec![vec![vec![INF; 2]; 3]; n];
    let mut from = vec![vec![vec![INF as usize; 2]; 3]; n];
    while let Some((Reverse(c), p, q, adr)) = que.pop() {
        if seen[p][q][1] != INF {
            continue;
        }
        
        if seen[p][q][0] == INF {
            seen[p][q][0] = c;
            from[p][q][0] = adr;
        } else {
            if from[p][q][0] == adr {
                continue;
            }
            seen[p][q][1] = c; 
            from[p][q][1] = adr;
        }
        
        if livehouse.contains(&p) && adr != p && q+1 <= 2 && seen[p][q+1][1] == INF {
            if q==1 {
            }
            que.push((Reverse(c), p, q+1, p));
        }

        for &(ni, w) in g[p].iter() {
            // liveしない
            if  seen[ni][q][1] == INF {
                que.push((Reverse(c+w), ni, q, adr));
            }
        }
    }
    println!("{}", seen[n-1][2][0]);
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
    }
}

