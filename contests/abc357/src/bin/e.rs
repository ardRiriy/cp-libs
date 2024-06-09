use proconio::input;
use proconio::marker::Usize1;

fn solve() {
    input! {
        n: usize,
        g: [Usize1; n]
    }

    let mut ans = vec![INF; n];
    let mut dept = vec![INF; n];
    let mut flag = (false, INF as usize);

    for i in 0..n {
        if ans[i] == INF {
            dept[i] = 1;
            dfs(i, &g, &mut ans, &mut dept, &mut flag);
        }
    }

    eprintln!(
        "{}",
        ans.iter()
            .enumerate()
            .map(|(i, v)| format!("{} : {}", i + 1, v))
            .collect::<Vec<String>>()
            .join("\n")
    );
    println!("{}", ans.iter().sum::<u64>());
}

fn dfs(
    pos: usize,
    g: &Vec<usize>,
    ans: &mut Vec<u64>,
    dept: &mut Vec<u64>,
    flag: &mut (bool, usize),
) {

    let next = g[pos];
    if ans[next] != INF {
        ans[pos] = ans[next] + 1;
        return;
    }

    if dept[next] != INF {
        flag.0 = next != pos;
        flag.1 = next;

        ans[pos] = dept[pos] + 1 - dept[next];
        return;
    } else {
        dept[next] = dept[pos] + 1;
        dfs(next, g, ans, dept, flag);

        if flag.0 {
            ans[pos] = ans[next];
            if flag.1 == pos {
                flag.0 = false;
            }
        } else {
            ans[pos] = ans[next] + 1;
        }
    }
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
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
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
