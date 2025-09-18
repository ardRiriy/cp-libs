use std::collections::VecDeque;

use library::utils::{consts::{DI, DJ, INF}, input::Input};

fn solve(ip: &mut Input) {
    let (h, w) = ip.pair::<usize, usize>();

    let a = ip.vector::<String>(h).iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (0..h).flat_map(|i| (0..w).map(move |j| (i, j)))
        .find(|(i, j)| a[*i][*j] == 'S')
        .unwrap();
    
    let mut seen = vec![vec![vec![INF; w]; h]; 2];
    seen[0][start.0][start.1] = 0;
    
    let mut que = VecDeque::new();
    que.push_back((0, start));
    
    while let Some((p, (i, j))) = que.pop_front() {
        for r in 0..4 {
            let ni = i.wrapping_add(DI[r]);
            let nj = j.wrapping_add(DJ[r]);
            
            if ni >= h || nj >= w || a[ni][nj] == '#' {
                continue;
            }
            
            if a[ni][nj] == '.' || a[ni][nj] == 'G' || a[ni][nj] == 'S' {
                if seen[p][ni][nj] == INF {
                    seen[p][ni][nj] = seen[p][i][j] + 1;
                    que.push_back((p, (ni, nj)));
                }
            } else if a[ni][nj] == '?' {
                if seen[1 - p][ni][nj] == INF {
                    seen[1 - p][ni][nj] = seen[p][i][j] + 1;
                    que.push_back((1 - p, (ni, nj)));
                }
            } else if a[ni][nj] == 'o' && p == 0 {
                if seen[p][ni][nj] == INF {
                    seen[p][ni][nj] = seen[p][i][j] + 1;
                    que.push_back((p, (ni, nj)));
                }
            } else if a[ni][nj] == 'x' && p == 1 {
                if seen[p][ni][nj] == INF {
                    seen[p][ni][nj] = seen[p][i][j] + 1;
                    que.push_back((p, (ni, nj)));
                }
            }
        }
    }
    
    let goal = (0..h).flat_map(|i| (0..w).map(move |j| (i, j)))
        .find(|(i, j)| a[*i][*j] == 'G')
        .unwrap();
    
    let ans = seen[0][goal.0][goal.1].min(seen[1][goal.0][goal.1]);
    println!("{}", if ans == INF { -1 } else { ans as i64 });

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
