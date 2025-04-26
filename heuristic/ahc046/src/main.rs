use std::{cmp::Ordering, collections::{BTreeSet, VecDeque}};

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use proconio::input;
use rand::Rng;
use utils::utils::{get_time, DC, DI, DJ};

mod data_structure;
mod simulated_annealing;
mod utils;

static TIME_LIMIT :f64 = 1.995;
static INF :i32 = 1<<30;

fn get_route(n: usize, from: (usize, usize), to: (usize, usize), board: &FixedBitSet) -> Result<Vec<(char, char)>, ()> {
    eprintln!("from, to: {:?}, {:?}", from, to);
    // 直前のindex, そのマスに到達するのに使った操作
    let mut visited_from = vec![vec![((INF as usize, INF as usize), '_', '_'); n]; n];
    let mut queue = VecDeque::new();
    queue.push_back(from);
    visited_from[from.0][from.1] = ((!0, !0), '@', '@');
    while let Some((i, j)) = queue.pop_front() {
        for r in 0..4 {
            let ni = i.wrapping_add(DI[r]);
            let nj = j.wrapping_add(DJ[r]);
            if ni >= n || nj >= n || board[ni * n + nj] {
                continue;
            }
            if visited_from[ni][nj].0.0 == INF as usize {
                // 移動
                visited_from[ni][nj] = ((i, j), 'M', DC[r]);
                queue.push_back((ni, nj));
            }
        }

        // 滑走
        // 壁があるか、外側ギリギリまで移動できる
        for r in 0..4 {
            let ni = i.wrapping_add(DI[r]);
            let nj = j.wrapping_add(DJ[r]);
            if ni >= n || nj >= n || board[ni * n + nj] {
                continue;
            }
            let mut ni2 = ni;
            let mut nj2 = nj;
            loop {
                let nni2 = ni2.wrapping_add(DI[r]);
                let nnj2 = nj2.wrapping_add(DJ[r]);
                if nni2 >= n || nnj2 >= n || board[nni2 * n + nnj2] {
                    break;
                }
                ni2 = nni2;
                nj2 = nnj2;
            }
            if visited_from[ni2][nj2].0.0 == INF as usize {
                // 滑走
                visited_from[ni2][nj2] = ((i, j), 'S', DC[(r+ 2) % 4]);
                queue.push_back((ni2, nj2));
            }
        }
    }

    // 操作の種類, 操作後の座標
    let mut route = vec![];
    if visited_from[to.0][to.1].0.0 == INF as usize {
        return Err(());
    }
    // ルートをたどる
    let mut cur = to;
    
    while cur != from {
        let (prev, op, dir) = visited_from[cur.0][cur.1];
        route.push((op, dir));
        cur = prev;
    }
    route.reverse();
    return Ok(route);
}

fn main() {
    get_time();
    input! {
        n: usize,
        m: usize,
        p: [(usize, usize); m],
    }

    let mut ans = vec![vec![]; m-1];
    let mut board = FixedBitSet::with_capacity(n * n);

    for (i, (&(from_i, from_j), &(to_i, to_j))) in p.iter().tuple_windows().enumerate() {
        let route = get_route(n, (from_i, from_j), (to_i, to_j), &board);
        ans[i] = route.unwrap();
    }


    // 操作の種類, 操作後の座標(配置なら配置した座標)
    let mut cur_ops = ans.iter()
        .map(|v| {
            v.iter()
                .map(|&ops| (ops.0, ops.1))
                .collect_vec()
        })
        .collect_vec();
    

    let mut rng = rand::thread_rng();
    'tl: while get_time() < TIME_LIMIT {
        // eprintln!("===============================");
        // eprintln!("cur_ops.len()={}", cur_ops.len());

        let i = rng.gen_range(0..cur_ops.len());
        // eprintln!("cur_ops[i].len()={}", cur_ops[i].len());
        let j = rng.gen_range(0..cur_ops[i].len());
        let r = rng.gen_range(0..4);
        if cur_ops[i][j].0 == 'S' {
            continue 'tl;
        }
        if cur_ops[i][j].0 == 'A' {
            cur_ops[i].remove(j);
        } else {
            // i, jにいる時点での座標を求める
            let mut pos = p[i];
            for l in 0..j {
                let (op, dir) = cur_ops[i][l];
                    
                if op == 'M' {
                    for r in 0..4 {
                        if DC[r] != dir {
                            continue;
                        }
                        pos.0 = pos.0.wrapping_add(DI[r]);
                        pos.1 = pos.1.wrapping_add(DJ[r]);
                        break;
                    }
                } else if op == 'S' {
                    // 壁に当たるまで
                    let mut ni = pos.0.wrapping_add(DI[r]);
                    let mut nj = pos.1.wrapping_add(DJ[r]);
                    loop {
                        if ni >= n || nj >= n || board[ni * n + nj] {
                            break;
                        }
                        ni = ni.wrapping_add(DI[r]);
                        nj = nj.wrapping_add(DJ[r]);
                    }
                    pos = (ni, nj);
                }
            }

            let ni = pos.0.wrapping_add(DI[r]);
            let nj = pos.1.wrapping_add(DJ[r]);
            if ni >= n || nj >= n{
                continue 'tl;
            }

            cur_ops[i].insert(j, ('A', DC[r]));
        }

        let mut board = FixedBitSet::with_capacity(n * n);
        let mut new_ops = vec![vec![]; m-1];
        for k in 0..m-1 {
        let mut pos = p[k];

            if k < i {
                new_ops[k] = cur_ops[k].clone();
                // baordの更新だけする
                for &(op, dir) in new_ops[k].iter() {
                    if op == 'M' {
                        for r in 0..4 {
                            if DC[r] != dir {
                                continue;
                            }
                            pos.0 = pos.0.wrapping_add(DI[r]);
                            pos.1 = pos.1.wrapping_add(DJ[r]);
                            break;
                        }
                        dbg!(pos);
                    } else if op == 'S' {
                        // 壁に当たるまで
                        let mut ni = pos.0.wrapping_add(DI[r]);
                        let mut nj = pos.1.wrapping_add(DJ[r]);
                        loop {
                            if ni >= n || nj >= n || board[ni * n + nj] {
                                break;
                            }
                            ni = ni.wrapping_add(DI[r]);
                            nj = nj.wrapping_add(DJ[r]);
                        }
                        pos = (ni, nj);
                        dbg!(pos);
                    } else {
                        let (mut ni, mut nj) = pos;
                        for r in 0..4 {
                            if DC[r] != dir {
                                continue;
                            }
                            ni = ni.wrapping_add(DI[r]);
                            nj = nj.wrapping_add(DJ[r]);
                            break;
                        }
                        eprintln!("{} {}", ni, nj);
                        eprintln!("{:?}", pos);
                        let new_val = !board[ni * n + nj];
                        board.set(ni*n+nj, new_val);
                    }
                }
            } else {
                let mut start = p[k];
                let mut end = p[k];
                for l in 0..cur_ops[k].len() {
                    let (op, dir) = cur_ops[k][l];
                    if op == 'A' {
                        if let Ok(route) = get_route(n, start, end, &board) {
                            for &val in route.iter() {
                                new_ops[k].push(val);
                                let (op, dir) = val;
                                if op == 'M' {
                                    for r in 0..4 {
                                        if DC[r] != dir {
                                            continue;
                                        }
                                        pos.0 = pos.0.wrapping_add(DI[r]);
                                        pos.1 = pos.1.wrapping_add(DJ[r]);
                                        break;
                                    }
                                } else if op == 'S' {
                                    // 壁に当たるまで
                                    let mut ni = pos.0.wrapping_add(DI[r]);
                                    let mut nj = pos.1.wrapping_add(DJ[r]);
                                    loop {
                                        if ni >= n || nj >= n || board[ni * n + nj] {
                                            break;
                                        }
                                        ni = ni.wrapping_add(DI[r]);
                                        nj = nj.wrapping_add(DJ[r]);
                                    }
                                }
                            }
                            start = end;
                        } else {
                            continue 'tl;
                        }
                        let mut ppp = end;
                        new_ops[k].push(('A', dir));
                        for r in 0..4 {
                            if DC[r] != dir {
                                continue;
                            }
                            ppp.0 = ppp.0.wrapping_add(DI[r]);
                            ppp.1 = ppp.1.wrapping_add(DJ[r]);
                            break;
                        }
                        // boardに書き込み
                        let new_val = !board[ppp.0 * n + ppp.1];
                        board.set(ppp.0 * n + ppp.1, new_val);
                    } else {
                        for r in 0..4 {
                            if DC[r] != dir {
                                continue;
                            }
                            end.0 = end.0.wrapping_add(DI[r]);
                            end.1 = end.1.wrapping_add(DJ[r]);
                        }
                    }
                }

                // startから次のマスまでのルートを取得
                if let Ok(route) = get_route(n, start, p[k+1], &board) {
                    for &val in route.iter() {
                        new_ops[k].push(val);
                        let (op, dir) = val;
                        if op == 'M' {
                            for r in 0..4 {
                                if DC[r] != dir {
                                    continue;
                                }
                                pos.0 = pos.0.wrapping_add(DI[r]);
                                pos.1 = pos.1.wrapping_add(DJ[r]);
                                break;
                            }
                        } else if op == 'S' {
                            // 壁に当たるまで
                            let mut ni = pos.0.wrapping_add(DI[r]);
                            let mut nj = pos.1.wrapping_add(DJ[r]);
                            loop {
                                if ni >= n || nj >= n || board[ni * n + nj] {
                                    break;
                                }
                                ni = ni.wrapping_add(DI[r]);
                                nj = nj.wrapping_add(DJ[r]);
                            }
                        }
                    }
                } else {
                    continue 'tl;
                }
            }
        }


        if new_ops.iter().flatten().count()<cur_ops.iter().flatten().count() {
            eprintln!("===============================");
            eprintln!("{}", new_ops.iter().flatten().map(|&x| format!("{} {}", x.0, x.1)).join("\n"));
            cur_ops = new_ops;
        }
    }

    println!("{}", cur_ops.iter().flatten().map(|&x| format!("{} {}", x.0, x.1)).join("\n"));
}