mod data_structure {
    #[allow(dead_code)]
    pub mod data_structure {
        use core::fmt;
        use std::fmt::Formatter;
    
        use itertools::Itertools;
        use rand::{thread_rng, Rng};
    
        // [0, N)の値の集合の管理をO(1)で判定するデータ構造
        // ref: https://topcoder-tomerun.hatenablog.jp/entry/2021/06/12/134643
        #[derive(Clone)]
        pub struct IndexSet {
            que: Vec<usize>,
            pos: Vec<Option<usize>>,
        }
    
        impl IndexSet {
            pub fn new(n: usize) -> Self {
                Self { que: vec![], pos: vec![None; n] }
            }
    
            // 値iを集合に追加する
            pub fn add(&mut self, i: usize) -> bool {
                if self.pos[i].is_some() {
                    return false;
                }
    
                self.pos[i] = Some(self.que.len());
                self.que.push(i);
                true
            }
    
            // 値iを集合から削除する
            pub fn remove(&mut self, i: usize) -> bool {
                if self.pos[i].is_none() {
                    return false;
                }
    
                let p = self.pos[i].unwrap();
                let q = *self.que.last().unwrap();
                self.que[p] = q;
                self.que.pop();
                self.pos[q] = Some(p);
                self.pos[i] = None;
                true
            }
    
            // 値iが集合に含まれるかどうかを判定
            pub fn contain(&self, i: usize) -> bool {
                self.pos[i].is_some()
            }
    
            // 集合に含まれる値からランダムに一つ選んで返す 
            pub fn random(&self) -> usize {
                let mut rng = thread_rng();
                self.que[rng.gen_range(0..self.que.len())]
            }
    
            pub fn size(&self) -> usize {
                self.que.len()
            }
        }
    
        impl std::fmt::Display for IndexSet {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let str = self.que.iter()
                    .sorted()
                    .join(" ");
                write!(f, "[ {} ]", str)
            }
        }
    
        impl std::fmt::Debug for IndexSet {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{}", self)
            }
        }
    }
}
mod simulated_annealing {
    #[allow(dead_code)]
    pub mod simulated_annealing {
        use std::{marker::PhantomData, ops::Sub};
    
        use num::ToPrimitive;
        use rand::{thread_rng, Rng};
    
        use crate::utils::utils::get_time;
    
        pub trait AnnealingState<T> {
            fn pre_process(&mut self);      // 焼きなまし実行前の準備
            fn eval(&self) -> Option<T>;    // evalした結果得られるスコアを返却。変更は行わない
            fn rollback(&mut self);         // 元に戻す処理など(reject)
            fn post_process(&mut self, new_score:T);     // accept
            fn current_score(&self) -> T;
        }
    
        pub struct SimulatedAnnealing<S, T>
        where S: AnnealingState<T>
        {
            pub state: S,
            start_temp: f64,
            end_temp: f64,
            _marker: PhantomData<T>, // 型Tを明示する
        }
    
        impl<S: AnnealingState<T>, T: Copy+ToPrimitive + Sub<Output = T>> SimulatedAnnealing<S, T> {
            pub fn new(
                state: S,
                start_temp: f64,
                end_temp: f64,
            ) -> Self {
                Self {
                    state,
                    start_temp,
                    end_temp,
                    _marker: PhantomData
                }
            }
    
            pub fn execute(&mut self, limit: f64) {
                let mut rng = thread_rng();
                while get_time() < limit {
                    self.state.pre_process();
                    if let Some(new_score) = self.state.eval() {
                        // TODO: 設定可能に変える
                        let temp = self.start_temp + (self.end_temp - self.start_temp) * (get_time() / limit); 
                        let prob = f64::exp((new_score - self.state.current_score()).to_f64().unwrap() / temp);
                        if prob < rng.gen_range(0.0..1.0) {
                            self.state.post_process(new_score);
                        } else {
                            self.state.rollback();
                        }
                    }
                }
            }
        }
    }
}
mod utils {
    #[allow(dead_code)]
    pub mod utils {
        #[inline]
        pub fn get_time() -> f64 {  // sec
            static mut STIME: f64 = -1.0;
            let t = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap();
            let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
            unsafe {
                if STIME < 0.0 {
                    STIME = ms;
                }
                ms - STIME
            }
        }
    }
}

use std::collections::VecDeque;
use std::io::{self, BufRead};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub k: usize,
    pub robots: Vec<(usize, usize)>,
    pub walls: WallMap,
}

#[derive(Debug, Clone)]
pub struct WallMap {
    n: usize,
    // vertical_walls[i][j]: (i,j)と(i,j+1)の間に壁があるか
    vertical_walls: Vec<Vec<bool>>,
    // horizontal_walls[i][j]: (i,j)と(i+1,j)の間に壁があるか  
    horizontal_walls: Vec<Vec<bool>>,
}

impl WallMap {
    pub fn new(n: usize, v_walls: Vec<String>, h_walls: Vec<String>) -> Self {
        let mut vertical_walls = vec![vec![false; n-1]; n];
        let mut horizontal_walls = vec![vec![false; n]; n-1];
        
        // 縦壁の情報を解析
        for (i, line) in v_walls.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                vertical_walls[i][j] = ch == '1';
            }
        }
        
        // 横壁の情報を解析
        for (i, line) in h_walls.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                horizontal_walls[i][j] = ch == '1';
            }
        }
        
        WallMap {
            n,
            vertical_walls,
            horizontal_walls,
        }
    }
    
    /// 2つの隣接するマス間に壁があるかを判定
    /// 隣接していない場合はNoneを返す
    pub fn has_wall_between(&self, pos1: (usize, usize), pos2: (usize, usize)) -> Option<bool> {
        let (i1, j1) = pos1;
        let (i2, j2) = pos2;
        
        // 縦方向の移動（同じ列）
        if j1 == j2 {
            if i1 + 1 == i2 {
                // pos1が上、pos2が下
                return Some(self.horizontal_walls[i1][j1]);
            } else if i2 + 1 == i1 {
                // pos2が上、pos1が下
                return Some(self.horizontal_walls[i2][j2]);
            }
        }
        
        // 横方向の移動（同じ行）
        if i1 == i2 {
            if j1 + 1 == j2 {
                // pos1が左、pos2が右
                return Some(self.vertical_walls[i1][j1]);
            } else if j2 + 1 == j1 {
                // pos2が左、pos1が右
                return Some(self.vertical_walls[i2][j2]);
            }
        }
        
        // 隣接していない
        None
    }
    
    /// 指定した位置から指定した方向に移動可能かを判定
    pub fn can_move(&self, pos: (usize, usize), dir: Direction) -> bool {
        let (i, j) = pos;
        
        match dir {
            Direction::Up => {
                if i == 0 { return false; }
                !self.horizontal_walls[i-1][j]
            },
            Direction::Down => {
                if i >= self.n - 1 { return false; }
                !self.horizontal_walls[i][j]
            },
            Direction::Left => {
                if j == 0 { return false; }
                !self.vertical_walls[i][j-1]
            },
            Direction::Right => {
                if j >= self.n - 1 { return false; }
                !self.vertical_walls[i][j]
            },
            Direction::Stay => true,
        }
    }
    
    /// 移動後の座標を返す（移動できない場合は元の座標）
    pub fn move_pos(&self, pos: (usize, usize), dir: Direction) -> (usize, usize) {
        if !self.can_move(pos, dir) {
            return pos;
        }
        
        let (i, j) = pos;
        match dir {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
            Direction::Stay => (i, j),
        }
    }
}

static DI: &[usize] = &[!0, 0, 1, 0];
static DJ: &[usize] = &[0, 1, 0, !0];
static DC : &[char] = &['U', 'R', 'D', 'L'];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stay,
}

impl Direction {
    pub fn to_char(&self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
            Direction::Stay => 'S',
        }
    }
    
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            'S' => Some(Direction::Stay),
            _ => None,
        }
    }
    
    pub fn from_usize(x: usize) -> Option<Self> {
        match x {
            0 => Some(Direction::Up),
            1 => Some(Direction::Right),
            2 => Some(Direction::Down),
            3 => Some(Direction::Left),
            _ => None,
        }
    }
}

// グローバル変数として入力を保持
pub static INPUT: Lazy<Input> = Lazy::new(|| {
    read_input()
});

fn read_input() -> Input {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // N M K を読み取り
    let first_line = lines.next().unwrap().unwrap();
    let nums: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (n, m, k) = (nums[0], nums[1], nums[2]);
    
    // ロボットの初期位置を読み取り
    let mut robots = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let pos: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        robots.push((pos[0], pos[1]));
    }
    
    // 縦壁の情報を読み取り（N行）
    let mut v_walls = Vec::new();
    for _ in 0..n {
        v_walls.push(lines.next().unwrap().unwrap());
    }
    
    // 横壁の情報を読み取り（N-1行）
    let mut h_walls = Vec::new();
    for _ in 0..n-1 {
        h_walls.push(lines.next().unwrap().unwrap());
    }
    
    let walls = WallMap::new(n, v_walls, h_walls);
    
    Input {
        n,
        m,
        k,
        robots,
        walls,
    }
}


// fn dfs(pos: (usize, usize), ops: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, ans: &mut Vec<(usize, usize)>, poses: &mut Vec<(usize, usize)>) {
    
//     let input = &*INPUT;
//     seen[pos.0][pos.1] = true;

//     let mut rng = rand::thread_rng();
//     let s = rng.gen_range(0..4);
//     //let s=0;

//     for r in 0..4 {
//         let r = (s+r) % 4;
//         let ni = pos.0.wrapping_add(DI[r]);
//         let nj = pos.1.wrapping_add(DJ[r]);

//         if ni >= input.n || nj >= input.n || seen[ni][nj] {
//             continue;
//         }
//         if input.walls.can_move(pos, Direction::from_usize(r).unwrap()) {
//             let idx = ops[0].iter().position(|c| *c == DC[r]).unwrap();
//             ans.push((0, idx));
//             poses.push((ni,nj));
//             dfs((ni, nj), ops, seen, ans, poses);
//             let idx = ops[0].iter().position(|c| *c == DC[(r + 2) % 4]).unwrap();
//             ans.push((1, idx));
//             poses.push(pos);
//         }
//     }
    
// }


fn find_seen_false_path(start: (usize, usize), ops: &Vec<Vec<char>>, seen: &Vec<Vec<bool>>) -> Vec<usize> {
    let input = &*INPUT;
    let mut from = vec![vec![None; input.n]; input.n];
    let mut goal = (!0, !0);
    let mut id = vec![vec![None; input.n]; input.n];

    let mut que = VecDeque::new();
    from[start.0][start.1] = Some((!0, !0));
    que.push_back(start);
    
    while let Some((i, j)) = que.pop_front() {
        if !seen[i][j] {
            goal = (i, j);
            break;
        }
        
        for r in 0..4 {
            let ni = i.wrapping_add(DI[r]);
            let nj = j.wrapping_add(DJ[r]);
            if ni >= input.n || nj >= input.n || from[ni][nj].is_some() {
                continue;
            }
            if input.walls.can_move((i, j), Direction::from_usize(r).unwrap()) {
                let idx = ops[0].iter().position(|&c| c == DC[r]).unwrap();
                id[ni][nj] = Some(idx);
                from[ni][nj] = Some((i,j));
                que.push_back((ni, nj));
            }
        }
    }

    
    let mut res = vec![];
    let mut cur = goal;
    loop {
        if id[cur.0][cur.1].is_none() {
            break;
        }
        res.push(id[cur.0][cur.1].unwrap());
        cur = from[cur.0][cur.1].unwrap();
    }

    res.reverse();
    res
}


fn main() {
    // INPUTにアクセスすると、初回アクセス時に入力が読み込まれる
    let input = &*INPUT;
    let mut rng = rand::thread_rng();
    
    eprintln!("N={}, M={}, K={}", input.n, input.m, input.k);
    eprintln!("Robots: {:?}", input.robots);
    
    fn transpose_ops(robot_ops: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let k = robot_ops[0].len();
        let m = robot_ops.len();
        
        (0..k).map(|button_idx| {
            (0..m).map(|robot_idx| {
                robot_ops[robot_idx][button_idx]
            }).collect()
        }).collect()
    }
    let ops = (0..input.m)
        .map(|_| {
            let mut r = vec!['L', 'L', 'R', 'R', 'U', 'U', 'D', 'D', 'S', 'S'];
            r.shuffle(&mut rng);
            r
        })
        .collect_vec();

    let mut seen = vec![vec![false; input.n]; input.n];
    for (i, j) in input.robots.iter() {
        seen[*i][*j] = true;
    }
    
    let mut ans = vec![];

    let mut robots_pos = input.robots.clone();

    while seen.iter().flatten().any(|&c| !c) {
        loop {
            if let Some(r) = (0..4).filter_map(|r| {
                let ni = robots_pos[0].0.wrapping_add(DI[r]);
                let nj = robots_pos[0].1.wrapping_add(DJ[r]);
                if ni < input.n && nj < input.n && !seen[ni][nj] && input.walls.can_move(robots_pos[0], Direction::from_usize(r).unwrap()) {
                    Some(r)
                } else {
                    None
                }
            })
            .choose(&mut rng) {
                let idx = ops[0].iter().position(|c| *c == DC[r]).unwrap();
                ans.push(idx);
                
                // robots_posを更新
                for robot_idx in 0..input.m {
                    if let Some(r) = DC.iter().position(|&c| c == ops[robot_idx][idx]) {
                        let ni = robots_pos[robot_idx].0.wrapping_add(DI[r]);
                        let nj = robots_pos[robot_idx].1.wrapping_add(DJ[r]);
                        
                        if ni < input.n && nj < input.n && input.walls.can_move(robots_pos[robot_idx], Direction::from_usize(r).unwrap()) {
                            robots_pos[robot_idx] = (ni, nj);
                            seen[ni][nj] = true;
                        }
                    }
                }
                
            } else {
                break;
            }
        }
        
        if seen.iter().flatten().all(|&c| c) {
            break;
        }
        
        // falseまでの最短経路を求める
        let v = find_seen_false_path(robots_pos[0], &ops, &seen);
        
        for idx in v.iter() {
            ans.push(*idx);
            for robot_idx in 0..input.m {
                    if let Some(r) = DC.iter().position(|&c| c == ops[robot_idx][*idx]) {
                        let ni = robots_pos[robot_idx].0.wrapping_add(DI[r]);
                        let nj = robots_pos[robot_idx].1.wrapping_add(DJ[r]);

                    if ni < input.n && nj < input.n && input.walls.can_move(robots_pos[robot_idx], Direction::from_usize(r).unwrap()) {
                        robots_pos[robot_idx] = (ni, nj);
                        seen[ni][nj] = true;
                    }
                }
            }
        }
        
        
        
        dbg!(&robots_pos);
    }
    

    let ops = transpose_ops(ops);
    println!("{}", ops.iter().map(|v| v.iter().join(" ")).join("\n"));
    println!("{}", ans.iter().map(|x| x.to_string()).join("\n"));
}
