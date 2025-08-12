use std::collections::VecDeque;

use library::utils::consts::INF;
use itertools::{iproduct, Itertools};
use proconio::{input, marker::Usize1};


fn dfs(p: usize, seen: &mut Vec<bool>, g: &Vec<Vec<usize>>, gn: &mut Vec<Vec<usize>>, k: usize) -> Vec<Vec<usize>> {
    seen[p] = true;
    let mut v = vec![];
    for &ni in g[p].iter() {
        if seen[ni] { continue; }
        let ret = dfs(ni, seen, g, gn, k);
        v.push(ret);
    }


    let mut res = vec![vec![]; k+1];

    for i in 0..v.len() {
        for j in i+1..v.len() {
            for kk in 1..k {
                for (x,y) in iproduct!(v[i][kk].iter(), v[j][k-kk].iter()) {
                    gn[*x].push(*y);
                    gn[*y].push(*x);
                }
            }
        }
    }

    

    for i in 0..v.len() {
        for j in 0..k {
            for &x in v[i][j].iter() {
                res[j+1].push(x);
            }
        }
        for &x in v[i][k].iter() {
            gn[p].push(x);
            gn[x].push(p);
        }
    }

    res[1].push(p);

    res
}


fn solve() {
    input! {
        n: usize,
        k: usize,
        e: [(Usize1, Usize1); n-1],
    }

    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u,v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });

    let mut gv = vec![vec![]; n];
    let _ = dfs(0,&mut vec![false; n], &g, &mut gv, k);
    
    //dbg!(&gv);


    let mut dist = vec![INF; n];
    dist[0] = 0;
    let mut que = VecDeque::new();
    que.push_back(0);
    while let Some(p) = que.pop_front() {
        for &ni in gv[p].iter() {
            if dist[ni] != INF { continue; }
            dist[ni] = dist[p] + 1;
            que.push_back(ni);
        }
    }

    println!("{}", dist[1..].iter().map(|x| if *x == INF { -1 } else { *x as i32}).join(" "));

}

fn main() {
    input!{
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}

