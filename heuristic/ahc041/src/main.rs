use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
use rand::{thread_rng, Rng};
use utils::get_time;

static TIME_LIMIT: f64 = 1.995;
static INF: i32 = 1 << 30;

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

fn calc_score(g: &Vec<Vec<usize>> ,v: &Vec<i32>, a: &Vec<u32>, h: u32) -> Result<u32, ()> {
    // TODO: 差分計算して高速化する
    let mut score = 0;
    for (i, _) in v.iter().enumerate().filter(|(_, &xi)| xi == -1) {
        let mut que = VecDeque::new();
        que.push_back((i, 0));
        while let Some((u, d)) = que.pop_front() {
            score += a[u] * (d+1);
            for &v in &g[u] {
                if d + 1 > h {
                    // invalid
                    return Err(());
                }
                que.push_back((v, d + 1));
            }
        }
    }

    Ok(1+score)
}

fn calc_score2(g: &mut Vec<Vec<usize>>, v: &mut Vec<i32>, a: &Vec<u32>, h: u32, depth: &Vec<u32>, i: usize, k: usize) -> Result<i32, ()> {
    if depth[k]+1 > h {
        return Err(());
    }
    // iの親をkに変更した場合のスコアの変動を計算
    let mut prev_score = 0;
    let mut next_score = 0;
    let mut que = VecDeque::new();
    que.push_back((i, depth[i]));

    while let Some((u, d)) = que.pop_front() {
        prev_score += a[u] * (d+1);
        for &v in &g[u] {
            que.push_back((v, d + 1));
        }
    }

    // 親の更新
    let prev = v[i];
    v[i] = k as i32;
    // gの更新
    if prev != -1 {
        g[prev as usize].retain(|&x| x != i);
    }
    g[k].push(i);
    
    // 更新後のスコア計算
    que.clear();
    que.push_back((i, depth[k]+1));
    while let Some((u, d)) = que.pop_front() {
        next_score += a[u] * (d+1);
        for &ni in &g[u] {
            if d + 1 > h {
                // invalid
                if prev != -1 {
                    g[prev as usize].push(i);
                }
                g[k].retain(|&x| x != i);
                v[i] = prev;
                return Err(());
            }
            que.push_back((ni, d + 1));
        }
    }

    // 復元
    if prev != -1 {
        g[prev as usize].push(i);
    }
    g[k].retain(|&x| x != i);
    v[i] = prev;

    Ok(next_score as i32 - prev_score as i32)
}

fn main() {
    get_time();
    input! {
        n: usize,
        m: usize,
        h: u32,
        a: [u32; n],
        edges: [(usize, usize); m],
    }

    let graph = edges.iter().fold(vec![vec![]; n], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    let mut seen = vec![INF; n];
    for (i, _) in a.iter().enumerate().sorted_unstable_by_key(|&(_, &x)| x) {
        if seen[i] != INF {
            continue;
        }
        seen[i] = -1;

        // bfs
        // ただし、深さdに対して、max(d/2, 1)個の辺のみを探索する
        let mut q = VecDeque::new();
        q.push_back((i, 0));
        while let Some((u, d)) = q.pop_front() {
            let mut cnt = 0;
            for &v in &graph[u] {
                if cnt >= (d / 2).max(1) {
                    break;
                }
                if seen[v] != INF {
                    continue;
                }
                seen[v] = u as i32;
                if d + 1 >= h {
                    continue;
                }
                cnt += 1;
                q.push_back((v, d + 1));
            }
        }
    }

    let mut g = vec![vec![]; n];
    for (i, &x) in seen.iter().enumerate() {
        if x == -1 {
            continue;
        }
        g[x as usize].push(i);
    }

    let mut depth = vec![0; n];
    for i in 0..n {
        if seen[i] != -1 {
            continue;
        }
        let mut que = VecDeque::new();
        que.push_back((i, 0));
        while let Some((u, d)) = que.pop_front() {
            depth[u] = d;
            for &v in &g[u] {
                que.push_back((v, d + 1));
            }
        }
    }

    let mut current_score = calc_score(&g,&seen, &a, h).unwrap();
    // eprintln!("init score: {}", current_score);

    let mut rng = thread_rng();

    let start_temp = 5e2;
    let end_temp = 1e-3;

    let mut cnt = 0;
    while get_time() < TIME_LIMIT {
        let i = rng.gen_range(0..n);
        cnt += 1;
        if cnt % 100000 == 0 {
            println!("{}", seen.iter().join(" "));
        }

        let j = rng.gen_range(0..graph[i].len());
        let k = graph[i][j];
        // iの親がk, あるいはkの子がiの場合はスキップ
        if seen[i] == k as i32 || seen[k] == i as i32 {
            continue;
        }

        if let Ok(score_diff) = calc_score2(&mut g, &mut seen, &a, h, &depth, i, k) {
            let temp = start_temp + (end_temp - start_temp) * get_time() / TIME_LIMIT;
            let prob = (score_diff as f64 / temp).exp();
            if score_diff > 0 || rng.gen::<f64>() < prob {
                // accept
                current_score = (current_score as i32 + score_diff) as u32;

                // グラフと深さの更新
                let prev = seen[i];
                seen[i] = k as i32;

                let mut que = VecDeque::new();
                que.push_back((i, depth[k]+1));
                while let Some((u, d)) = que.pop_front() {
                    depth[u] = d;
                    for &v in &g[u] {
                        que.push_back((v, d + 1));
                    }
                }

                g[k].push(i);
                if prev != -1 {
                    g[prev as usize].retain(|&x| x != i);
                }
            }
        }
    }

    println!("{}", seen.iter().join(" "));
}
