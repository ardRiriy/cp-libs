use itertools::iproduct;
use proconio::input;

fn solve() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        dfs(n)
            .iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn dfs(level: usize) -> Vec<Vec<char>> {
    if level == 1 {
        let mut res = vec![vec!['#'; 3]; 3];
        res[1][1] = '.';
        return res;
    } else if level == 0 {
        return vec![vec!['#'; 1]; 1];
    }

    let mut res = vec![vec!['.'; 3usize.pow(level as u32)]; 3usize.pow(level as u32)];

    let elem = dfs(level - 1);
    let k = 3usize.pow(level as u32 - 1);
    for (si, sj) in iproduct!(0..3, 0..3) {
        if si == 1 && sj == 1 {
            continue;
        }

        for (i, j) in iproduct!(0..k, 0..k) {
            res[si * k + i][sj * k + j] = elem[i][j];
        }
    }
    res
}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/

static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        if *self > elm {
            *self = elm;
            true
        } else {
            false
        }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
            *self = elm;
            true
        } else {
            false
        }
    }
}

fn main() {
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
