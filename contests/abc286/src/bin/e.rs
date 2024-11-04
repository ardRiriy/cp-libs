use proconio::{input, marker::{Chars, Usize1}};
fn main() {
    input!{
        n: usize,
        a: [u64; n],
        s: [Chars; n],
        q: usize,
        queries: [(Usize1, Usize1); q]
    }

    let g = s.iter()
        .map(|v| v.iter()
            .enumerate()
            .filter_map(|(idx, &c)| if c == 'Y' { Some((idx, (1, a[idx]))) } else { None })
            .collect::<Vec<(usize, (u64, u64))>>()
        ).collect();

    let dist = warshall_floyd(&g);

    for (from, to) in queries {
        if dist[from][to].0 == 1 << 60 {
            println!("Impossible");
        } else {
            println!("{} {}", dist[from][to].0, dist[from][to].1 + a[from]);
        }
    }
}

fn warshall_floyd(graph: &Vec<Vec<(usize, (u64, u64))>>) -> Vec<Vec<(u64, u64)>>
{
    // 頂点数V, 辺の本数Eに対してO(|V|^3 + |E|)
    let inf = 1 << 60;
    let v = graph.len();
    let mut distance = vec![vec![(inf, inf); v]; v];
    for i in 0..v {
        for (to, (cost, w)) in graph[i].iter() {
            distance[i][*to] = (*cost, *w);
        }
        distance[i][i] = (0, 0);
    }

    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                if distance[i][k].0 == inf || distance[k][j].0 == inf {
                    continue;
                }
                // distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                if distance[i][j].0 > distance[i][k].0 + distance[k][j].0 {
                    distance[i][j].0 = distance[i][k].0 + distance[k][j].0;
                    distance[i][j].1 = distance[i][k].1 + distance[k][j].1;
                } else if distance[i][j].0 == distance[i][k].0 + distance[k][j].0 && distance[i][j].1 < distance[i][k].1 + distance[k][j].1 {
                    distance[i][j].1 = distance[i][k].1 + distance[k][j].1;
                }
            }
        }
    }

    distance
}
