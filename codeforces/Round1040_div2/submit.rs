use library::{data_structure::segment_tree::{Monoid, SegmentTree}, utils::{chlibs::ChLibs, consts::INF, input::Input}};

struct Sum;
impl Monoid for Sum {
    type S = usize;

    fn op(a: Self::S, b: Self::S) -> Self::S {
        a+b
    }

    fn id() -> Self::S {
        0
    }
}

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();

    let a = ip.vector::<u32>(n).iter().map(|x| x-1).collect::<Vec<_>>();
    let mut seg :SegmentTree<Sum> = SegmentTree::from_vec(&vec![1; n]);
    let mut a = a.iter()
        .enumerate()
        .map(|(i,&ai)| (ai, i))
        .collect::<Vec<_>>();
    a.sort();
    
    let mut dp = vec![INF; 2];
    dp[0] = 0;
    
    for i in 0..n {
        let (_, p) = a[i];
        let idx = seg.get(..p);
        let jdx = seg.get(p+1..);
        
        let mut ndp = vec![INF; 2];
        for j in 0..2 {
            if dp[j] != INF {
                ndp[0].chmin(dp[j] + idx as u64);
                ndp[1].chmin(dp[j] + jdx as u64);
            }
        }

        dp = ndp; 
        seg.set(p, 0);
    }

    println!("{}", dp.iter().min().unwrap());
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
    pub mod data_structure {
        pub mod segment_tree {
            use std::{
                fmt::{self, Debug, Formatter},
                ops::{Bound, RangeBounds},
            };
            pub trait Monoid {
                type S: Copy;
                fn op(a: Self::S, b: Self::S) -> Self::S;
                fn id() -> Self::S;
            }
            pub struct SegmentTree<T: Monoid> {
                n: usize,
                len: usize,
                data: Vec<T::S>,
            }
            impl<T: Monoid> SegmentTree<T> {
                pub fn new(len: usize) -> Self {
                    let mut n = 1;
                    while n < len {
                        n <<= 1;
                    }
                    let data = vec![T::id(); 2 * n];
                    Self { n, len, data }
                }
                pub fn from_vec(vec: &Vec<T::S>) -> Self {
                    let len = vec.len();
                    let mut seg = Self::new(len);
                    seg.data[seg.n..][..len].copy_from_slice(&vec);
                    for i in (1..seg.n).rev() {
                        seg.data[i] = T::op(seg.data[i << 1], seg.data[(i << 1) + 1]);
                    }
                    seg
                }
                pub fn set(&mut self, index: usize, value: T::S) {
                    assert!(index < self.len);
                    let mut cur = index + self.n;
                    self.data[cur] = value;
                    while cur > 1 {
                        cur >>= 1;
                        self.data[cur] = T::op(
                            self.data[cur << 1],
                            self.data[(cur << 1) + 1],
                        );
                    }
                }
                pub fn get<R>(&self, range: R) -> T::S
                where
                    R: RangeBounds<usize>,
                {
                    let l = match range.start_bound() {
                        Bound::Included(&s) => s,
                        Bound::Excluded(&s) => s + 1,
                        Bound::Unbounded => 0,
                    };
                    let r = match range.end_bound() {
                        Bound::Included(&e) => e + 1,
                        Bound::Excluded(&e) => e,
                        Bound::Unbounded => self.len,
                    };
                    self.get_rec(1, 0, self.n, l, r)
                }
                fn get_rec(
                    &self,
                    cur: usize,
                    seg_l: usize,
                    seg_r: usize,
                    q_l: usize,
                    q_r: usize,
                ) -> T::S {
                    if seg_r <= q_l || q_r <= seg_l {
                        return T::id();
                    }
                    if q_l <= seg_l && seg_r <= q_r {
                        return self.data[cur];
                    }
                    let mid = (seg_l + seg_r) >> 1;
                    let left = self.get_rec(cur << 1, seg_l, mid, q_l, q_r);
                    let right = self.get_rec((cur << 1) + 1, mid, seg_r, q_l, q_r);
                    T::op(left, right)
                }
            }
            impl<T> Debug for SegmentTree<T>
            where
                T: Monoid,
                T::S: Debug,
            {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    writeln!(f, "")?;
                    let total = self.data.len() - 1;
                    if total == 0 {
                        return write!(f, "(empty)");
                    }
                    let leaves = (total + 1) / 2;
                    debug_assert!(leaves.is_power_of_two());
                    let h = leaves.trailing_zeros() as usize;
                    let repr: Vec<String> = self
                        .data
                        .iter()
                        .skip(1)
                        .map(|v| format!("{:?}", v))
                        .collect();
                    let w = repr.iter().map(|s| s.len()).max().unwrap();
                    let sep = 1;
                    let unit = w + sep;
                    let line_len = leaves * unit - sep;
                    let mut idx = 0;
                    for d in 0..=h {
                        let nodes = 1 << d;
                        let block = 1 << (h - d);
                        let stride = block * unit;
                        let mut line = vec![' '; line_len];
                        for i in 0..nodes {
                            if idx >= repr.len() {
                                break;
                            }
                            let s = &repr[idx];
                            let center = i * stride + stride / 2;
                            let start = center - s.len() / 2;
                            for (j, ch) in s.chars().enumerate() {
                                let pos = start + j;
                                if pos < line_len {
                                    line[pos] = ch;
                                }
                            }
                            idx += 1;
                        }
                        let end = line
                            .iter()
                            .rposition(|&c| c != ' ')
                            .map_or(0, |p| p + 1);
                        writeln!(f, "{}", line[..end].iter().collect::< String > ())?;
                    }
                    Ok(())
                }
            }
        }
    }
    pub mod utils {
        pub mod chlibs {
            pub trait ChLibs<T: std::cmp::Ord> {
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
        }
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

