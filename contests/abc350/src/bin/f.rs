use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
use rand::Rng;

#[derive(Debug)]
struct Crane {
    pos: (usize, usize),
    is_hold_container: u64,
    size: u8,
    operate: Vec<char>,
    visited: Vec<(usize, bool)>,
}

impl Crane {
    fn new(n: usize) -> Crane {
        Crane {
            pos: (n, 0),
            is_hold_container: EMPTY,
            operate: vec![],
            size: if n == 0 { 0 } else { 1 },
            visited: vec![(EMPTY as usize, false); 25],
        }
    }
    fn destroy(&mut self) {
        if self.is_hold_container != EMPTY {
            return;
        }
        self.pos = (EMPTY as usize, EMPTY as usize);
        self.operate.push('B');
    }
    fn move_to_r(&mut self, r: usize) {
        let tmp = self.pos;
        self.pos = (
            self.pos.0.wrapping_add(DI[r]),
            self.pos.1.wrapping_add(DJ[r]),
        );

        if self.visited[self.pos.0 * 5 + self.pos.1].0 + 1 == self.operate.len()
            && self.visited[self.pos.0 * 5 + self.pos.1].1 == (self.is_hold_container == EMPTY)
            && self.size == 1
        {
            self.pos = tmp;
            self.operate.push('.');
            return;
        }

        self.operate.push(DD[r]);

        self.visited[self.pos.0 * 5 + self.pos.1] =
            (self.operate.len(), self.is_hold_container == EMPTY)
    }
    fn release_container(&mut self, board: &mut Vec<Vec<u64>>) {
        board[self.pos.0][self.pos.1] = self.is_hold_container;
        self.is_hold_container = EMPTY;
        self.operate.push('Q');
    }
    fn grab_container(&mut self, board: &mut Vec<Vec<u64>>) {
        self.is_hold_container = board[self.pos.0][self.pos.1];
        board[self.pos.0][self.pos.1] = EMPTY;
        self.operate.push('P');
    }
}

struct Board {
    area: Vec<Vec<u64>>,
    cranes: Vec<Crane>,
    wait_containers: Vec<Vec<u64>>,
    offed_containers: Vec<Vec<bool>>,
}

impl Board {
    fn new(n: usize, containers: Vec<Vec<u64>>, cranes: Vec<Crane>) -> Board {
        let mut res = Board {
            area: vec![vec![EMPTY; n]; n],
            cranes,
            wait_containers: containers
                .iter()
                .map(|v| {
                    let mut v = v.clone();
                    v.reverse();
                    v
                })
                .collect_vec(),
            offed_containers: vec![vec![false; n]; n],
        };
        res.next_move();
        res
    }

    fn next_move(&mut self) {
        let n = self.area.len();
        for i in 0..n {
            // (i, 0)にコンテナがなく、かつコンテナを保持しているクレーンがない場合に搬入が行われる
            if self.area[i][0] == EMPTY
                && !self
                    .cranes
                    .iter()
                    .filter_map(|c| {
                        if c.is_hold_container == EMPTY {
                            None
                        } else {
                            Some(c.pos)
                        }
                    })
                    .any(|(pi, pj)| pi == i && pj == 0)
            {
                match self.wait_containers[i].pop() {
                    Some(x) => self.area[i][0] = x,
                    None => {}
                };
            }

            if self.area[i][n - 1] != EMPTY {
                let x = self.area[i][n - 1] as usize;
                self.offed_containers[x / n][x % n] = true;
                self.area[i][n - 1] = EMPTY;
            }
        }
    }

    // 次に搬出したいコンテナを返す。
    fn target(&self) -> Vec<usize> {
        self.offed_containers
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| match v.iter().enumerate().find(|&(_, b)| !*b) {
                Some((i, _)) => Some(idx * self.offed_containers.len() + i),
                None => None,
            })
            .collect_vec()
    }

    // クレーンの番号と行き先のコンテナに対して最短経路を1つ求める。他のコンテナが障害物扱い。
    fn route(&self, s: (usize, usize), g: (usize, usize), is_small: bool) -> Vec<usize> {
        let inf: usize = 1usize << 61 + 1;
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        que.push_back(s);

        let n: usize = self.area.len();
        let mut bd: Vec<Vec<usize>> = vec![vec![inf; n]; n];
        // eprintln!("{:?}", self.cranes[idx]);
        bd[s.0][s.1] = !0;
        while let Some((pi, pj)) = que.pop_front() {
            for r in 0..4 {
                let ni: usize = pi.wrapping_add(DI[r]);
                let nj: usize = pj.wrapping_add(DJ[r]);
                if ni < n && nj < n {
                    if bd[ni][nj] != inf
                        || self
                            .cranes
                            .iter()
                            .enumerate()
                            .map(|(i, c)| c.pos)
                            .any(|(i, j)| i == ni && j == nj)
                        || (is_small && self.area[ni][nj] != EMPTY)
                    {
                        // すでに到達済みの頂点、あるいは他のコンテナが存在するマスへの移動は禁止
                        continue;
                    }
                    bd[ni][nj] = r;
                    que.push_back((ni, nj));
                }
            }
        }

        if bd[g.0][g.1] == inf {
            return Vec::new();
        }

        //gi,gjからルートを逆算
        let mut pos: (usize, usize) = g;
        let mut res: Vec<usize> = vec![];
        while bd[pos.0][pos.1] != !0 {
            res.push(bd[pos.0][pos.1]);
            pos = (
                pos.0.wrapping_sub(DI[bd[pos.0][pos.1]]),
                pos.1.wrapping_sub(DJ[bd[pos.0][pos.1]]),
            )
        }
        res.reverse();
        res
    }

    fn move_container_to_next(&mut self, idx: usize, lock: &mut Vec<bool>) {
        let N = self.area.len();
        for j in (0..N - 2) {
            for i in 0..N {
                if lock[i * N + j] {
                    continue;
                }
                if self.area[i][j] != EMPTY && self.area[i][j + 1] == EMPTY {
                    if (i, j) == self.cranes[idx].pos {
                        // eprintln!("grab on move_container_to_next");
                        self.cranes[idx].grab_container(&mut self.area);
                        return;
                    } else {
                        let route = self.route(self.cranes[idx].pos, (i, j), false);
                        if !route.is_empty() {
                            self.cranes[idx].move_to_r(route[0]);
                            lock[i * N + j] = true;
                            return;
                        }
                    }
                }
            }
        }
        for r in 0..4 {
            let next = (
                self.cranes[idx].pos.0.wrapping_add(DI[r]),
                self.cranes[idx].pos.1.wrapping_add(DJ[r]),
            );
            if next.0 < 5 && next.1 < 5 {
                let route = self.route(
                    self.cranes[idx].pos,
                    (
                        self.cranes[idx].pos.0.wrapping_add(DI[r]),
                        self.cranes[idx].pos.1.wrapping_add(DJ[r]),
                    ),
                    false,
                );
                if !route.is_empty() {
                    self.cranes[idx].move_to_r(route[0]);
                    return;
                }
            }
        }
    }

    // 大クレーンの操作を行う。
    fn large_crane(&mut self, lock: &mut Vec<bool>) {
        let N = self.area.len();

        if self.cranes[0].is_hold_container == EMPTY {
            let dist: Vec<usize> = self
                .target()
                .iter()
                .map(|&x| {
                    if let Some((p, _)) = self
                        .area
                        .iter()
                        .flatten()
                        .enumerate()
                        .find(|(_, y)| **y == x as u64)
                    {
                        (p / N).abs_diff(self.cranes[0].pos.0)
                            + (p % N).abs_diff(self.cranes[0].pos.1)
                    } else {
                        for idx in 0..self.wait_containers.len() {
                            for jdx in 0..self.wait_containers[idx].len() {
                                if self.wait_containers[idx][jdx] == x as u64 {
                                    return 100 + jdx + self.cranes[0].pos.0 + self.cranes[0].pos.1;
                                }
                            }
                        }
                        1e18 as usize
                    }
                })
                .collect_vec();
            if dist.iter().all(|x| *x == 1e18 as usize) {
                self.cranes[0].operate.push('.');
                return;
            }

            let next_container =
                self.target()[dist.iter().enumerate().min_by_key(|&(_, x)| *x).unwrap().0];
            lock[next_container] = true;

            let pos: (usize, usize) = if let Some((p, _)) = self
                .area
                .iter()
                .flatten()
                .enumerate()
                .find(|(_, x)| **x == next_container as u64)
            {
                // 盤面上に存在する場合
                // eprintln!("found target: {next_container}, pos: {:?}", (p / N, p % N));
                (p / N, p % N)
            } else {
                // 盤面上にコンテナが存在しない場合は、next_containerを取り出す
                let res = self
                    .wait_containers
                    .iter()
                    .enumerate()
                    .find(|&(_, v)| v.iter().any(|x| *x == next_container as u64))
                    .unwrap()
                    .0;

                (res, 0)
            };

            if pos == self.cranes[0].pos {
                // 同じマスにいる場合、持ち上げる
                self.cranes[0].is_hold_container = self.area[pos.0][pos.1];
                self.cranes[0].operate.push('P');
                self.area[pos.0][pos.1] = EMPTY;
            } else {
                let route: Vec<usize> = self.route(self.cranes[0].pos, pos, false);
                if !route.is_empty() {
                    self.cranes[0].move_to_r(route[0]);
                } else {
                    let mut rng = rand::thread_rng();
                    let r = rng.gen_range(0..4);
                    let ni = self.cranes[0].pos.0.wrapping_add(DI[r]);
                    let nj = self.cranes[0].pos.1.wrapping_add(DJ[r]);
                    if ni < N && nj < N && !self.cranes.iter().any(|c| c.pos == (ni, nj)) {
                        self.cranes[0].move_to_r(r);
                    } else {
                        self.cranes[0].operate.push('.');
                    }
                }
            }
        } else {
            // 持っているコンテナがtargetに含まれるなら排出口へ、そうでないならそれ以外のEMPTYマスへ
            let goal: (usize, usize) = if let Some(_) = self
                .target()
                .iter()
                .find(|&&x| x == self.cranes[0].is_hold_container as usize)
            {
                (self.cranes[0].is_hold_container as usize / N, N - 1)
            } else {
                // もよりのEmptyマスを一つ見つける
                {
                    let mut que: VecDeque<(usize, usize)> = VecDeque::from([self.cranes[0].pos]);
                    let mut seen: Vec<Vec<bool>> = vec![vec![false; N - 1]; N];
                    let mut res: (usize, usize) = (1e18 as usize, 0);
                    while let Some((pi, pj)) = que.pop_front() {
                        if seen[pi][pj] {
                            continue;
                        }

                        seen[pi][pj] = true;
                        if self.area[pi][pj] == EMPTY && pj != 0 {
                            res = (pi, pj);
                            break;
                        }
                        for r in 0..4 {
                            let ni: usize = pi.wrapping_add(DI[r]);
                            let nj: usize = pj.wrapping_add(DJ[r]);
                            if ni < N && nj < N - 1 && nj != 0 && !seen[ni][nj] {
                                que.push_back((ni, nj));
                            }
                        }
                    }
                    if res.0 == 1e18 as usize {
                        panic!("not found empty grid");
                    }
                    res
                }
            };
            // eprintln!("goal: {:?}", goal);
            // eprintln!("crane: {:?}", board.cranes[i]);

            if self.cranes[0].pos == goal {
                // 排出口に到達したらそこでコンテナを離す
                if goal.1 == N - 1 {
                    // eprintln!("搬出: {}", board.cranes[i].is_hold_container);
                    self.offed_containers[self.cranes[0].is_hold_container as usize / N]
                        [self.cranes[0].is_hold_container as usize % N] = true;
                }
                self.area[goal.0][goal.1] = self.cranes[0].is_hold_container;
                self.cranes[0].is_hold_container = EMPTY;
                self.cranes[0].operate.push('Q');
            } else {
                let route: Vec<usize> = self.route(self.cranes[0].pos, goal, false);
                if !route.is_empty() {
                    self.cranes[0].move_to_r(route[0]);
                } else {
                    let mut rng = rand::thread_rng();
                    let r = rng.gen_range(0..4);
                    let ni = self.cranes[0].pos.0.wrapping_add(DI[r]);
                    let nj = self.cranes[0].pos.1.wrapping_add(DJ[r]);
                    if ni < N && nj < N && !self.cranes.iter().any(|c| c.pos == (ni, nj)) {
                        self.cranes[0].move_to_r(r);
                    } else {
                        self.cranes[0].operate.push('.');
                    }
                }
            }
        }
    }
}

static EMPTY: u64 = 1u64 << 60 + 3;
static DI: &[usize] = &[1, 0, !0, 0];
static DJ: &[usize] = &[0, 1, 0, !0];
static DD: &[char] = &['D', 'R', 'U', 'L'];

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [[u64; N]; N]
    }

    let cranes: Vec<Crane> = (0..N).into_iter().map(|i| Crane::new(i)).collect_vec();
    let mut board = Board::new(N, A, cranes);

    // 大クレーン、小クレーン1つを運用する
    for i in 0..board.cranes.len() - 1 {
        if board.cranes[i].size == 1 {
            board.cranes[i].operate.push('B');
            board.cranes[i].pos = (EMPTY as usize, EMPTY as usize);
        }
    }
    let mut turn = 0;

    loop {
        eprintln!("---------------------------------------------");
        eprintln!("turn: {turn}");

        // debug output
        debugger(&board);

        // 小クレーン同士が、同じクレーンを対象に取らないようにする
        let mut lock = vec![false; N * N];
        board.large_crane(&mut lock);

        let targets = board.target();
        eprintln!("targets: {:?}", targets);

        for i in 1..N {
            if board.cranes[i].pos.0 == EMPTY as usize {
                continue;
            }

            if board.cranes[i].is_hold_container == EMPTY {
                // 小クレーンは、盤面上のコンテナのみに興味を持てる
                // 搬出可能なコンテナがあれば、それに向かう
                if let Some((container_pos, _)) =
                    board
                        .area
                        .iter()
                        .flatten()
                        .enumerate()
                        .find(|&(container_pos, x)| {
                            targets.iter().any(|t| *t == *x as usize)
                                && !lock[container_pos]
                                && !board
                                    .route(
                                        (container_pos / N, container_pos % N),
                                        (*x as usize / N, N - 1),
                                        true,
                                    )
                                    .is_empty()
                        })
                {
                    let route = board.route(
                        board.cranes[i].pos,
                        (container_pos / N, container_pos % N),
                        false,
                    );
                    if (container_pos / N, container_pos % N) == board.cranes[i].pos {
                        board.cranes[i].grab_container(&mut board.area);
                        continue;
                    } else if !route.is_empty() {
                        lock[container_pos] = true;

                        board.cranes[i].move_to_r(route[0]);

                        continue;
                    }
                }

                board.move_container_to_next(i, &mut lock);
            } else {
                let x: usize = board.cranes[i].is_hold_container as usize;
                let destination: (usize, usize) = if targets.iter().any(|t| *t == x) {
                    (x / N, N - 1)
                } else {
                    (EMPTY as usize, (N - 1) - x % N)
                };

                if board.cranes[i].pos == destination {
                    board.cranes[i].release_container(&mut board.area);
                } else {
                    if destination.0 != EMPTY as usize {
                        let route = board.route(board.cranes[i].pos, destination, true);
                        if !route.is_empty() {
                            board.cranes[i].move_to_r(route[0]);
                        } else if board.cranes[i].pos.1 < N - 2
                            && board.area[board.cranes[i].pos.0][board.cranes[i].pos.1 + 1] == EMPTY
                            && !board.cranes.iter().any(|c| {
                                c.pos.0 == board.cranes[i].pos.0
                                    && c.pos.1 == board.cranes[i].pos.1 + 1
                            })
                        {
                            board.cranes[i].move_to_r(1);
                        } else {
                            board.cranes[i].release_container(&mut board.area);
                        }
                    } else if board.cranes[i].pos.1 < N - 2
                        && board.area[board.cranes[i].pos.0][board.cranes[i].pos.1 + 1] == EMPTY
                        && !board.cranes.iter().any(|c| {
                            c.pos.0 == board.cranes[i].pos.0 && c.pos.1 == board.cranes[i].pos.1 + 1
                        })
                    {
                        board.cranes[i].move_to_r(1);
                    } else {
                        board.cranes[i].release_container(&mut board.area);
                    }
                }
            }
        }
        // 全操作が終了したら、ターンの進行処理を行う
        board.next_move();

        // もし待機コンテナと盤面上のコンテナが存在しない場合は終了
        if board.offed_containers.iter().flatten().all(|b| *b) {
            break;
        }

        turn += 1;
        // for _ in 0..5e8 as usize {}
    }

    println!(
        "{}",
        board
            .cranes
            .iter()
            .map(|c| c.operate.iter().join(""))
            .join("\n")
    );
}

fn debugger(board: &Board) {
    eprintln!(
        "盤面: \n{}\n",
        board
            .area
            .iter()
            .map(|v| v
                .iter()
                .map(|u| if *u == EMPTY {
                    "__".to_string()
                } else {
                    format!("{:2}", u)
                })
                .join(" "))
            .join("\n")
    );
    eprintln!(
        "待機列: 左が先搬入\n{}\n",
        board
            .wait_containers
            .iter()
            .map(|v| v.iter().join(" "))
            .join("\n")
    );
    eprintln!(
        "current crane pos\n{}",
        board
            .cranes
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| if c.pos.0 != EMPTY as usize {
                Some(format!(
                    "{idx}: {:?} 所持コンテナ:{}",
                    c.pos, c.is_hold_container
                ))
            } else {
                None
            })
            .join("\n")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_target() {
        let mut board = Board {
            area: vec![vec![]],
            offed_containers: vec![vec![false; 5]; 5],
            cranes: vec![],
            wait_containers: vec![],
        };
        for i in 0..4 {
            board.offed_containers[0][i] = true;
        }
        assert_eq!(board.target(), vec![4, 5, 10, 15, 20]);
    }
}
