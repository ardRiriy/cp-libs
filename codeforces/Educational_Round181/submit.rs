use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let t = input.next::<usize>();
    for _ in 0..t {
        solve(&mut input);
    }
}

fn solve(input: &mut Input) {
    let (l, r) = input.pair::<usize, usize>();
    let mut ans = 0;
    ans += get(r, 2) - get(l - 1, 2);
    ans += get(r, 3) - get(l - 1, 3);
    ans += get(r, 5) - get(l - 1, 5);
    ans += get(r, 7) - get(l - 1, 7);

    ans += get(r, 2*3*5) - get(l - 1, 2*3*5);
    ans += get(r, 2*3*7) - get(l - 1, 2*3*7);
    ans += get(r, 2*5*7) - get(l - 1, 2*5*7);
    ans += get(r, 3*5*7) - get(l - 1, 3*5*7);

    ans -= get(r,6) - get(l - 1, 6);
    ans -= get(r, 10) - get(l - 1, 10);
    ans -= get(r, 14) - get(l - 1, 14);
    ans -= get(r, 15) - get(l - 1, 15);
    ans -= get(r, 21) - get(l - 1, 21);
    ans -= get(r, 35) - get(l - 1, 35);

    ans -= get(r, 2*3*5*7) - get(l - 1, 2*3*5*7);
    println!("{}", r-l+1-ans);
}

fn get(n: usize, k: usize) -> usize {
    // N以下の値であって、kの倍数の個数
    n / k
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
                pub fn weighted_graph(
                    &mut self,
                    n: usize,
                    m: usize,
                    is_one_way: bool,
                ) -> Vec<Vec<(usize, usize)>> {
                    let mut graph = vec![Vec::new(); n];
                    for _ in 0..m {
                        let (u, v, w): (usize, usize, usize) = self.triple();
                        graph[u - 1].push((v - 1, w));
                        if !is_one_way {
                            graph[v - 1].push((u - 1, w));
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

