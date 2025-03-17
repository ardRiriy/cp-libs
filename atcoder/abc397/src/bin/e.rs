use std::collections::{BTreeMap, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        k: usize,
        e: [(Usize1, Usize1); n*k-1],
    }
    let mut ins = vec![0; n*k];
    let g = e.iter()
        .fold(vec![vec![]; n*k], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            ins[u]+=1;
            ins[v]+=1;
            g
        });

    let mut que = VecDeque::new();
    let mut sets: Vec<Vec<usize>> = vec![vec![]; n*k];

    for i in 0..n*k {
        if ins[i] == 1 {
            que.push_back(i);
        }
    }
    
    while let Some(p) = que.pop_front() {
        let mut next = 0;
        // 書き込み
        if sets[p].len() == 0 {
            next = 1 % k;
        } else if sets[p].len() == 1 {
            // もちあげる
            next = (sets[p][0] + 1) % k;
        } else if sets[p].len() == 2 {
            if sets[p][0] + sets[p][1] + 1 == k {
                next = 0;
            } else {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
        
        for &ni in g[p].iter() {
            if ins[ni] > 1 {
                ins[ni] -= 1;
                if ins[ni] == 1 {
                    que.push_back(ni);
                }
                if next != 0 {
                    sets[ni].push(next);
                }
            }
        }        
    }
    
    println!("Yes");
}

