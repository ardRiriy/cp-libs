use amplify::confinement::Collection;
use itertools::iproduct;
use proconio::input;

mod data_structure;
mod simulated_annealing;
mod utils;

static TIME_LIMIT :f64 = 1.995;
static INF_I32: i32 = 1<<30;

/*
N=20, x<=10^3, above<=3*10^4
問題
N*Nの2次元グリッドがあります。上からi行目、左からj番目のマスをます(i,j)と呼びます。

いま、各マスには荷物が配置されています。ある荷物重さはw_{i,j}であり、その上には合計で$above_{i,j}$だけ乗せることが出来ます。

さて、各マスについて独立に、以下の問に答えてください。
- マス{i,j}から始めてマス{0,0}へ移動させるとき、運べる荷物の数の最大値を求めよ

return: 操作列？
*/
fn dp(n: usize, w: &Vec<Vec<i32>>, d: &Vec<Vec<i32>>) -> Vec<(i32, char)> {
    // dp[i][j][k] := (i,j)から(0,0)へ移動する際に、k個の荷物を運ぶときの、残り上に載せられる(荷物*距離)の最大値
    let mut dp = vec![vec![vec![-1; 2*n+1]; n]; n];
    let mut from = vec![vec![vec![(!0,!0,!0); 2*n+1]; n]; n];
    
    // 後ろから見る
    dp[n-1][n-1][0] = INF_I32-1;
    for (i, j, k) in iproduct!((0..n).rev(), (0..n).rev(), 0..=2*n) {
        if dp[i][j][k] == INF_I32 {
            continue;
        }

        if i > 0 {
            // 何も載せずに遷移
            if dp[i-1][j][k] < dp[i][j][k] {
                dp[i-1][j][k] = dp[i][j][k];
                from[i-1][j][k] = (i,j,k);
            }

            // 今の場所にある荷物を乗せて遷移
            // (i+j)*w[i][j] > dp[i][j][k]ならば駄目
            if (i+j+1) as i32 * w[i][j] <= dp[i][j][k] && w[i][j] > 0 {
                let val = (dp[i][j][k] - (i+j+1) as i32 * w[i][j]).min(d[i][j]);
                if dp[i-1][j][k+1] < val {
                    dp[i-1][j][k+1] = val;
                    from[i-1][j][k+1] = (i,j,k);
                }
            }
        }
        if j > 0 {
            if dp[i][j-1][k] < dp[i][j][k] {
                dp[i][j-1][k] = dp[i][j][k];
                from[i][j-1][k] = (i,j,k);
            }

            if (i+j+1) as i32 * w[i][j] <= dp[i][j][k] && w[i][j] > 0 {
                let val = (dp[i][j][k] - (i+j+1) as i32 * w[i][j]).min(d[i][j]);
                if dp[i][j-1][k+1] < val {
                    dp[i][j-1][k+1] = val;
                    from[i][j-1][k+1] = (i,j,k);
                }
            }
        }
    }

    // 最終的にkが最大になったところからbacktrack
    let mut stk = vec![];
    let mut res = vec![];
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    for kk in (0..=2*n).rev() {
        if dp[0][0][kk] != -1 {
            k = kk;
            break;
        }
    }
    assert_ne!(k, 0);
    while from[i][j][k].0 != !0 {
        let prev = from[i][j][k];
        // 移動を積む
        // 逆向き
        if prev.0 == i + 1{
            stk.push((3,'U'));
        } else {
            stk.push((3,'L'));
        }

        // kが増えたならそこまで行く必要があったので、resに積む
        if k != prev.2 {
            for (i,c) in stk.iter() {
                res.push((*i, *c));
            }
            res.push((1, '-'));

            stk.clear();
        }
        i = prev.0;
        j = prev.1;
        k = prev.2;
    }


    res.reverse();
    res
}

fn main() {
    input! {
        n: usize,
        mut w: [[i32; n]; n],
        d: [[i32; n]; n],
    }

    // w: 0なら箱なし
    // d: 耐久度 dpにだけ使う

    while w.iter().flatten().any(|val| val>&0) {
        let res = dp(n, &w, &d);
        if res.is_empty() {
            break;
        }
        let i = res.iter().filter(|(_, c)| *c == 'U').count();
        let j = res.iter().filter(|(_, c)| *c == 'L').count();
        // i,jから始める
        let mut cur_i = i;
        let mut cur_j = j;
        for _ in 0..i {
            println!("D");
        }
        for _ in 0..j {
            println!("R");
        }
        for (k, c) in res.iter() {
            if k == &1 {
                w[cur_i][cur_j] = 0;
                println!("{}", k);
            } else {
                if *c == 'U' {
                    cur_i -= 1;
                } else {
                    cur_j -= 1;
                }
                println!("{}", c);
            }
        }
    }
}
