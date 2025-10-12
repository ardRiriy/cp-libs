use std::collections::BinaryHeap;

use library::utils::{input::Input, iterlibs::collect::CollectIter};

fn check(i: usize, j: usize, s: &Vec<Vec<char>>) -> bool {
        let mut res = true;
        for di in 0..2 {
            for dj in 0..2 {
                if i+di >= s.len() || j+dj >= s[0].len() {
                    res = false;
                } else {
                    res = res && s[i+di][j+dj] == '#';
                }
            }
        }

        res
}

fn solve(ip: &mut Input) {
    let (h, w) = ip.pair::<usize>();
    let mut s = (0..h).map(|_| ip.chars()).collect_vec();
    
    let mut cnt = vec![vec![0; w]; h];
    
    for i in 0..h {
        for j in 0..w {
            if check(i,j,&s) {
                for di in 0..2 {
                    for dj in 0..2 {
                        cnt[i+di][j+dj] += 1;
                    }
                }
            }
        }
    }
    
    let mut pq = BinaryHeap::new();
    for i in 0..h {
        for j in 0..w {
            if cnt[i][j] > 0 {
                pq.push((cnt[i][j], (i, j)));
            }
        }
    }

    let mut ans = 0;
    while let Some((c, (i, j))) = pq.pop() {
        if c != cnt[i][j] {
            if cnt[i][j] > 0 {
                pq.push((cnt[i][j], (i,j)));
            }
            continue;
        }
        

        if i > 0 && j > 0 && check(i-1,j-1, &s) {
            cnt[i-1][j-1] -= 1;
            cnt[i][j-1] -= 1;
            cnt[i-1][j] -= 1;
            cnt[i][j] -= 1;
        }
        if i > 0 && check(i-1, j, &s) {
            cnt[i-1][j] -= 1;
            cnt[i][j] -= 1;
            cnt[i-1][j+1] -= 1;
            cnt[i][j+1] -= 1;
        }
        if j > 0 && check(i, j-1, &s) {
            cnt[i][j-1] -= 1;
            cnt[i][j] -= 1;
            cnt[i+1][j-1] -= 1;
            cnt[i+1][j] -= 1;
        }
        if check(i,j,&s) {
            cnt[i][j] -= 1;
            cnt[i+1][j] -= 1;
            cnt[i][j+1] -= 1;
            cnt[i+1][j+1] -= 1;
        }
        s[i][j] = '.';
        ans += 1;
    }

    println!("{}", ans);
}

fn main() {
    static IS_MULTI_TEST_CASE :bool = true;
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
