use cps::consts::{DI, DJ};
use proconio::{input, marker::Chars};

fn dfs(i: usize, j: usize, seen: &mut Vec<Vec<bool>>, s: &Vec<Vec<char>>, dpt: usize, k: usize) -> usize {
    if dpt == k {
        return 1;
    }

    let mut res = 0;
    let h = s.len();
    let w = s[0].len();
    for r in 0..4 {
        let ni = i.wrapping_add(DI[r]);
        let nj = j.wrapping_add(DJ[r]);

        if ni < h && nj < w && s[ni][nj] == '.' && !seen[ni][nj] {
            seen[ni][nj] = true;
            res += dfs(ni, nj, seen, s, dpt+1, k);
            seen[ni][nj] = false;
        }
    }

    res
}

fn main() {
    input!{
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h]
    }

    let mut ans = 0;
    let mut seen = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                seen[i][j] = true;
                ans += dfs(i, j, &mut seen, &s, 0, k);
                seen[i][j] = false;
            }
        }
    }
    println!("{}", ans);
}
