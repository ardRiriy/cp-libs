use library::{cumulative_sum::CumulativeSum, utils::input::Input};

fn solve(ip: &mut Input) {
    
    let n = ip.next::<usize>();
    let v = (0..n)
        .map(|_| ip.triple::<u64,u64,u64>())
        .collect::<Vec<_>>();
    
    let b = v.iter()
        .map(|&(_, _, b)| b) 
        .collect::<Vec<_>>();

    let csum = CumulativeSum::new(&b);
    
    static M: u64 = 1000;
    // dp[i][j] := i番目にテンションがjで始めたときの最終的な値 
    let mut dp = vec![vec![0; M as usize+1]; n+1];
    dp[n] = (0..=M).collect::<Vec<u64>>();

    for i in (0..n).rev() {
        for j in 0..=M {
            if j <= v[i].0 {
                dp[i][j as usize] = dp[i+1][(j + v[i].1) as usize];
            } else {
                let k = if j >= v[i].2 { j - v[i].2 } else { 0 };
                dp[i][j as usize] = dp[i+1][k as usize];
            }
        }
    }
    

    let q = ip.next::<usize>();
    for _ in 0..q {
        let x = ip.next::<u64>();
        
        let i = if x <= 500 {
            0
        } else {
            let targ = x-500;
            csum.binary_search(targ).unwrap_or_else(|x| x)
        };

        if i == n+1 {
            println!("{}", x - csum.get(..));
        } else {
            let s = x - csum.get(..i);
            println!("{}", dp[i][s as usize]);
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
    pub mod cumulative_sum {
        use crate::library::utils::integer::Integer;
        #[derive(Debug)]
        pub struct CumulativeSum<T> {
            sum: Vec<T>,
        }
        impl<T> CumulativeSum<T>
        where
            T: Integer,
        {
            pub fn new(init_vec: &Vec<T>) -> CumulativeSum<T> {
                let sum = init_vec
                    .iter()
                    .enumerate()
                    .fold(
                        vec![T::zero()],
                        |mut sum, (idx, x)| {
                            let next: T = sum[idx].clone() + x.clone();
                            sum.push(next);
                            sum
                        },
                    );
                CumulativeSum { sum }
            }
            pub fn get<R>(&self, range: R) -> T
            where
                R: std::ops::RangeBounds<usize>,
            {
                let start = match range.start_bound() {
                    std::ops::Bound::Included(&s) => s,
                    std::ops::Bound::Excluded(&s) => s + 1,
                    std::ops::Bound::Unbounded => 0,
                };
                let end = match range.end_bound() {
                    std::ops::Bound::Included(&e) => e + 1,
                    std::ops::Bound::Excluded(&e) => e,
                    std::ops::Bound::Unbounded => self.sum.len() - 1,
                };
                self.sum[end].clone() - self.sum[start].clone()
            }
            pub fn binary_search(&self, key: T) -> Result<usize, usize> {
                self.sum.binary_search(&key)
            }
        }
        mod tests {
            #[test]
            fn cumulative_sum_get() {
                use super::CumulativeSum;
                let v = vec![1, 3, 8];
                let cs = CumulativeSum::new(&v);
                assert_eq!(1 + 3 + 8, cs.get(0..3));
                assert_eq!(1 + 3, cs.get(0..2));
                assert_eq!(3 + 8, cs.get(1..3));
                assert_eq!(3, cs.get(1..2));
                assert_eq!(1 + 3 + 8, cs.get(0..= 2));
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

