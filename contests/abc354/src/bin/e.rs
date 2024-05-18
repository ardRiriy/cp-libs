use std::collections::BTreeMap;

use proconio::input;

fn solve() {
    input! {
        n: usize,
        a: [(u64, u64); n]
    }

    let mut dp = vec![2; 1 << n];
    dp[0] = 1;

    for idx in 0..1usize << n {
        let mut res = 1;
        for i in 0..n {
            for j in i + 1..n {
                // i, jが同じではないならばskip
                if !(a[i].0 == a[j].0 || a[i].1 == a[j].1) {
                    continue;
                }

                // i, jのいずれかが除かれていればskip
                if (idx >> i) & 1 == 0 || (idx >> j) & 1 == 0 {
                    continue;
                }

                // i, jを除いたあとの盤面が青木くんの勝ち盤面である場合、
                // i, jを除く前の盤面から始めれば高橋くんの勝ち盤面になる
                if dp[idx ^ (1 << i) ^ (1 << j)] == 1 {
                    res = 0
                }
            }
        }
        dp[idx] = res;
    }
    println!(
        "{}",
        if dp.pop().unwrap() == 0 {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}

fn solve2() {
    input! {
        n: usize,
        a: [(usize, usize); n]
    }

    fn lp(
        a: &Vec<(usize, usize)>,
        state: usize,
        turn: i32,
        memo: &mut BTreeMap<usize, i32>,
    ) -> i32 {
        if let Some(x) = memo.get(&state) {
            return *x;
        }

        let mut res = 1 - turn;

        'i: for i in 0..a.len() {
            for j in i + 1..a.len() {
                if a[i].0 != a[j].0 && a[i].1 != a[j].1 {
                    continue;
                }

                // 0 -> ari 1 -> nasi
                if (state >> i) & 1 == 1 || (state >> j) & 1 == 1 {
                    continue;
                }

                let kekka = lp(a, state ^ (1 << i) ^ (1 << j), 1 - turn, memo);
                if kekka == turn {
                    res = kekka;
                    break 'i;
                }
            }
        }

        memo.insert(state, res);
        res
    }

    println!(
        "{}",
        if lp(&a, 0, 0, &mut BTreeMap::new()) == 1 {
            "Aoki"
        } else {
            "Takahashi"
        }
    );
}
/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/

static INF: u64 = 1e18 as u64;

fn main() {
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve2();
        i -= 1;
    }
}
