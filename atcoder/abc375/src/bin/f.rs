use cps::warshall_floyd::{DefaultWFelm, WFelm, WarshallFloyd};
use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        q: usize,
        edges: [(Usize1, Usize1, u64); m],
    }
    let mut check = vec![true; m];
    
    // クエリを逆順から見ると辺追加クエリとみなせて、O(N^2)でできる
    let queries = (0..q).map(|_| {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    i: Usize1,
                }
                check[i] = false;
                (t, i, 0)
            },
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1
                }
                (t, x, y)
            },
            _ => unreachable!()
        }
    }).collect_vec();

    let g = edges.iter()
        .enumerate()
        .filter_map(|(i, &e)| if check[i] { Some(e) } else { None } )
        .fold(vec![vec![]; n], |mut g, (u, v, w)| {
            g[u].push((v, w));
            g[v].push((u, w));
            g
        });

    let mut ans = vec![];
    let mut wf = WarshallFloyd::new(&g, DefaultWFelm);

    for &(t, x, y) in queries.iter().rev() {
        if t == 1 {
            wf.add(edges[x].0, edges[x].1, edges[x].2);
        } else {
            ans.push(wf.get(x, y));
        }
    }

    println!("{}", ans.iter().rev().map(|x| if *x == DefaultWFelm.infinity() { -1 } else { *x as i64 }).join("\n"));
}

