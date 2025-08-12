use library::{graph::warshall_floyd::{DefaultWFelm, WFelm, WarshallFloyd}, utils::input::Input};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize, usize>();
    let mut g = ip.weighted_graph::<u64>(n, m, false, true);
    g.push(vec![]);
    
    let (k, t) = ip.pair::<usize, u64>();
    let d = ip.vector::<usize>(k).iter().map(|i| *i-1).collect::<Vec<_>>();
    for i in 0..k {
        g[d[i]].push((n, t));
        g[n].push((d[i], 0));
    }

    let mut wf = WarshallFloyd::new(&g, DefaultWFelm);
    let elm= DefaultWFelm;

    let q = ip.next();
    for _ in 0..q {
        let qt :i32 = ip.next();

        if qt == 1 {
            let (x,y,w) = ip.triple::<usize,usize,u64>();
            wf.add(x-1, y-1, w);
            wf.add(y-1, x-1, w);
        } else if qt == 2 {
            let x :usize = ip.next();
            let x = x-1;

            wf.add(x, n, t);
            wf.add(n, x, 0);
        } else {
            let mut res = 0;

            for i in 0..n {
                for j in 0..n {
                    let ret = wf.get(i, j);
                    if ret != elm.infinity() {
                        res += ret;
                    }
                }
            }
            println!("{}", res);
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
    pub mod graph {
        pub mod warshall_floyd {
            use crate::library::utils::integer::Integer;
            pub trait WFelm<T>
            where
                T: Copy,
            {
                fn min(&self, a: T, b: T) -> T;
                fn add(&self, a: T, b: T) -> T;
                fn infinity(&self) -> T;
                fn identity(&self) -> T;
            }
            pub struct DefaultWFelm;
            impl<T> WFelm<T> for DefaultWFelm
            where
                T: Integer,
            {
                fn min(&self, a: T, b: T) -> T {
                    a.min(b)
                }
                fn add(&self, a: T, b: T) -> T {
                    if a == self.infinity() || b == self.infinity() {
                        self.infinity()
                    } else {
                        a + b
                    }
                }
                fn infinity(&self) -> T {
                    T::inf()
                }
                fn identity(&self) -> T {
                    T::zero()
                }
            }
            pub struct WarshallFloyd<T, O>
            where
                T: Copy,
                O: WFelm<T>,
            {
                dist: Vec<Vec<T>>,
                op: O,
            }
            impl<T, O> WarshallFloyd<T, O>
            where
                T: Copy,
                O: WFelm<T>,
            {
                pub fn new(graph: &Vec<Vec<(usize, T)>>, op: O) -> Self {
                    let n = graph.len();
                    let mut dist = vec![vec![op.infinity(); n]; n];
                    for i in 0..n {
                        for (to, cost) in graph[i].iter() {
                            dist[i][*to] = op.min(dist[i][*to], *cost);
                        }
                        dist[i][i] = op.identity();
                    }
                    for k in 0..n {
                        for i in 0..n {
                            for j in 0..n {
                                dist[i][j] = op
                                    .min(dist[i][j], op.add(dist[i][k], dist[k][j]));
                            }
                        }
                    }
                    Self { dist, op }
                }
                pub fn get(&self, from: usize, to: usize) -> T {
                    self.dist[from][to]
                }
                pub fn add(&mut self, x: usize, y: usize, w: T) {
                    self.dist[x][y] = self.op.min(self.dist[x][y], w);
                    for &k in &[x, y] {
                        for i in 0..self.dist.len() {
                            for j in 0..self.dist.len() {
                                self.dist[i][j] = self
                                    .op
                                    .min(
                                        self.dist[i][j],
                                        self.op.add(self.dist[i][k], self.dist[k][j]),
                                    );
                            }
                        }
                    }
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
        pub mod integer {
            use std::{
                convert::TryFrom,
                ops::{
                    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign,
                    Shr, Sub, SubAssign,
                },
            };
            pub trait Integer: Copy + Default + Ord + Add<
                    Output = Self,
                > + AddAssign + Sub<
                    Output = Self,
                > + SubAssign + Mul<
                    Output = Self,
                > + MulAssign + Div<
                    Output = Self,
                > + DivAssign + Rem<
                    Output = Self,
                > + RemAssign + TryFrom<
                    i32,
                > + Shr<u32, Output = Self> + Not<Output = Self> {
                const MAX: Self;
                #[inline(always)]
                fn zero() -> Self {
                    Self::default()
                }
                #[inline(always)]
                fn inf() -> Self {
                    Self::MAX >> 2
                }
                #[inline(always)]
                fn from_i32(val: i32) -> Self {
                    Self::try_from(val)
                        .unwrap_or_else(|_| {
                            panic!(
                                "Cannot convert {} to {}", val, std::any::type_name::< Self
                                > ()
                            )
                        })
                }
            }
            macro_rules! impl_int {
                ($($t:ty),*) => {
                    $(impl Integer for $t { const MAX : Self = <$t >::MAX; })*
                };
            }
            impl_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
        }
    }
}

