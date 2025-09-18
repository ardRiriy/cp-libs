use ac_library::Dsu;
use library::{data_structure::unionfind::UnionFind, utils::input::Input};

// ???????????

fn solve(ip: &mut Input) {
    let (n, q) = ip.pair::<usize, usize>();


    let mut uf = Dsu::new(n);
    let mut cur = vec![0u64; n];
    let mut sum = vec![0; n];

    for _ in 0..q {
        let t :u8 = ip.next();
        if t == 1 {
            let (u, v) = ip.pair::<usize, usize>();
            let pu = sum[uf.leader(u-1)];
            let pv = sum[uf.leader(v-1)];
            if uf.same(u-1, v-1) {
                continue;
            }
            sum[uf.leader(u-1)] = 0;
            sum[uf.leader(v-1)] = 0;

            uf.merge(u-1, v-1);
            
            sum[uf.leader(u-1)] = pu + pv;
        } else if t == 2 {

            let u = ip.next::<usize>() - 1;
            sum[uf.leader(u)] -= cur[u];

            cur[u] = 1 - cur[u];
            sum[uf.leader(u)] += cur[u];

        } else {
            let u = ip.next::<usize>() - 1;
            println!("{}", if sum[uf.leader(u)] == 0 { "No" } else { "Yes" });
        }
    }
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
    pub mod data_structure {
        pub mod unionfind {
            #[derive(Debug)]
            pub struct UnionFind<T, F>
            where
                F: Fn(&T, &T) -> T,
            {
                vertex: Vec<usize>,
                data: Vec<Option<T>>,
                merge_op: F,
            }
            impl<T: Clone, F: Fn(&T, &T) -> T> UnionFind<T, F> {
                pub fn new(size: usize, merge_op: F) -> Self {
                    UnionFind {
                        vertex: vec![! 1; size],
                        data: vec![None; size],
                        merge_op,
                    }
                }
                pub fn leader(&mut self, u: usize) -> usize {
                    let elm: usize = self.vertex[u];
                    if elm > self.vertex.len() {
                        u
                    } else {
                        self.vertex[u] = self.leader(elm);
                        self.vertex[u]
                    }
                }
                pub fn same(&mut self, u: usize, v: usize) -> bool {
                    self.leader(u) == self.leader(v)
                }
                pub fn size(&mut self, u: usize) -> usize {
                    let idx: usize = self.leader(u);
                    !self.vertex[idx]
                }
                pub fn merge(&mut self, u: usize, v: usize) -> bool {
                    if self.same(u, v) {
                        return false;
                    }
                    let u_leader = self.leader(u);
                    let u_size = self.size(u);
                    let v_leader = self.leader(v);
                    let v_size = self.size(v);
                    let merged_size = !(u_size + v_size);
                    if u_size < v_size {
                        self.vertex[v_leader] = merged_size;
                        self.vertex[u_leader] = v_leader;
                        self.data[v_leader] = match (
                            &self.data[u_leader],
                            &self.data[v_leader],
                        ) {
                            (Some(du), Some(dv)) => Some((self.merge_op)(du, dv)),
                            (None, None) => None,
                            _ => {
                                unreachable!();
                            }
                        };
                    } else {
                        self.vertex[u_leader] = merged_size;
                        self.vertex[v_leader] = u_leader;
                        self.data[u_leader] = match (
                            &self.data[u_leader],
                            &self.data[v_leader],
                        ) {
                            (Some(du), Some(dv)) => Some((self.merge_op)(du, dv)),
                            (None, None) => None,
                            _ => {
                                unreachable!();
                            }
                        };
                    }
                    true
                }
                pub fn insert_data(&mut self, u: usize, value: T) {
                    let root = self.leader(u);
                    self.data[root] = Some(value);
                }
                pub fn get_data(&mut self, u: usize) -> Option<&T> {
                    let root = self.leader(u);
                    self.data[root].as_ref()
                }
            }
        }
    }
    pub mod utils {
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

