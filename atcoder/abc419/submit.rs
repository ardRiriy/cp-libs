use ac_library::ModInt998244353 as Mint;
use library::{algorithm::aho_corasick::AhoCorasick, utils::input::Input};

fn solve(ip: &mut Input) {
    let (n, l) = ip.pair::<usize, usize>();
    let s = (0..n).map(|_| ip.next::<String>()).collect::<Vec<_>>();
    
    let ahocora = AhoCorasick::new(&s);

    let m = ahocora.node_size();
    let mut dp = vec![vec![Mint::new(0); m]; 1<<n];
    
    dp[0][0] = Mint::new(1);
    
    for _i in 0..l {
        let mut ndp = vec![vec![Mint::new(0); m]; 1<<n];
        for st in 0..1<<n {
            for j in 0..m {
               // if dp[st][j] == Mint::new(0) {
               //     continue;
               // }
                let next_ids = ahocora.destination_node_ids_from_id(j);
                
                for id in next_ids  {
                    let outputs = ahocora.nodes[id].outputs.iter()
                        .map(|i| 1<<i)
                        .sum::<usize>();
                    
                    ndp[st | outputs][id] += dp[st][j];
                }
            }

        }
        dp = ndp;
    }

    println!("{}", dp[(1<<n)-1].iter().sum::<Mint>());
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
    pub mod algorithm {
        pub mod aho_corasick {
            use std::collections::{HashMap, VecDeque};
            #[derive(Debug, Clone)]
            pub struct Node {
                children: HashMap<char, usize>,
                failure: usize,
                pub outputs: Vec<usize>,
            }
            impl Node {
                fn new() -> Self {
                    Self {
                        children: HashMap::new(),
                        failure: 0,
                        outputs: Vec::new(),
                    }
                }
            }
            pub struct AhoCorasick {
                pub nodes: Vec<Node>,
                patterns: Vec<String>,
                goto: Vec<Vec<usize>>,
            }
            impl AhoCorasick {
                pub fn new(patterns: &Vec<String>) -> Self {
                    let mut res = Self {
                        nodes: vec![Node::new()],
                        patterns: patterns.clone(),
                        goto: Vec::new(),
                    };
                    res.build_trie();
                    res.build_failure();
                    res.build_goto();
                    res
                }
                fn build_trie(&mut self) {
                    for (pattern_idx, pattern) in self.patterns.iter().enumerate() {
                        let mut cur = 0;
                        for ch in pattern.chars() {
                            let nxt = if let Some(nxt) = self
                                .nodes[cur]
                                .children
                                .get(&ch)
                            {
                                *nxt
                            } else {
                                let new_idx = self.nodes.len();
                                self.nodes.push(Node::new());
                                self.nodes[cur].children.insert(ch, new_idx);
                                new_idx
                            };
                            cur = nxt;
                        }
                        self.nodes[cur].outputs.push(pattern_idx);
                    }
                }
                fn build_failure(&mut self) {
                    let mut que = VecDeque::new();
                    que.push_back(0);
                    while let Some(cur) = que.pop_front() {
                        for (&ch, &child) in &self.nodes[cur].children.clone() {
                            que.push_back(child);
                            let mut failure_node = self.nodes[cur].failure;
                            while failure_node != 0
                                && !self.nodes[failure_node].children.contains_key(&ch)
                            {
                                failure_node = self.nodes[failure_node].failure;
                            }
                            if let Some(&nxt) = self
                                .nodes[failure_node]
                                .children
                                .get(&ch)
                            {
                                if nxt != child {
                                    self.nodes[child].failure = nxt;
                                    let ext = self.nodes[nxt].outputs.clone();
                                    self.nodes[child].outputs.extend(ext);
                                }
                            }
                        }
                    }
                }
                fn build_goto(&mut self) {
                    self.goto = vec![vec![0; 26]; self.nodes.len()];
                    for node_id in 0..self.nodes.len() {
                        for (i, c) in ('a'..='z').enumerate() {
                            let mut cur = node_id;
                            while cur != 0 && !self.nodes[cur].children.contains_key(&c)
                            {
                                cur = self.nodes[cur].failure;
                            }
                            if let Some(&nxt) = self.nodes[cur].children.get(&c) {
                                self.goto[node_id][i] = nxt;
                            } else {
                                self.goto[node_id][i] = 0;
                            }
                        }
                    }
                }
                pub fn search(&self, s: &String) -> Vec<(usize, usize)> {
                    let mut res = Vec::new();
                    let mut cur = 0;
                    for (p, ch) in s.char_indices() {
                        while cur != 0 && !self.nodes[cur].children.contains_key(&ch) {
                            cur = self.nodes[cur].failure;
                        }
                        if let Some(&nxt) = self.nodes[cur].children.get(&ch) {
                            cur = nxt;
                        }
                        for &pattern_idx in &self.nodes[cur].outputs {
                            let start = p + 1 - self.patterns[pattern_idx].len();
                            res.push((pattern_idx, start));
                        }
                    }
                    res
                }
                pub fn node_size(&self) -> usize {
                    self.nodes.len()
                }
                pub fn destination_node_ids_from_str(&self, s: &String) -> Vec<usize> {
                    let mut cur = 0;
                    for ch in s.chars() {
                        while cur != 0 && !self.nodes[cur].children.contains_key(&ch) {
                            cur = self.nodes[cur].failure;
                        }
                        if let Some(&nxt) = self.nodes[cur].children.get(&ch) {
                            cur = nxt;
                        } else {
                            continue;
                        }
                    }
                    self.destination_node_ids_from_id(cur)
                }
                pub fn destination_node_ids_from_id(&self, id: usize) -> Vec<usize> {
                    self.goto[id].clone()
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

