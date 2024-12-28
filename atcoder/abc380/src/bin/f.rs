use itertools::Itertools;
use proconio::input;

fn dfs(t: usize, a: usize, memo: &mut Vec<Vec<i32>>, v: &Vec<u64>) -> i32 {
    if t == 0 {
        // 先手負け
        memo[t][a] = 1;
        return memo[t][a];
    }
    if memo[t][a] != 2 {
        return memo[t][a];
    }

    for i in 0..v.len() {
        // tのi bit目が1ならtを場に出すことが可能}
        if (t >> i) & 1 != 1 {
            continue;
        }

        // ibit目を場に出した
        let nt = t - (1 << i);

        // ret = 1(次のターンの先手視点で後手勝ち)があればこのターンでは0
        let ret = dfs(a, nt, memo, v);
        if ret == 1 {
            // 先手勝ち
            memo[t][a] = 0;
            return memo[t][a];
        }

        for j in 0..v.len() {
            if i == j {
                continue;
            }
            if (t >> j) & 1 == 1 || (a >> j) & 1 == 1 {
                // どちらかの手札にあるカードを対象に取れない
                continue;
            }
            if v[j] >= v[i] {
                // より大きい値は取れない
                continue;
            }

            let nt = nt | (1 << j);
            let ret = dfs(a, nt, memo, v);
            if ret == 1 {
                memo[t][a] = 0;
                return memo[t][a];
            }
        }
    }

    memo[t][a] = 1;
    return memo[t][a]
}


fn main() {
    input!{
        n: usize,
        m: usize,
        l: usize,
        a: [u64; n],
        b: [u64; m],
        c: [u64; l],
    }
    let s = n + m + l;
    let v = vec![a, b, c].iter().flatten().copied().collect_vec();
    let mut f = 0; 
    let mut sec = 0;
    for i in 0..n {
        f = f | 1 << i;
    }
    for i in 0..m {
        sec = sec | 1 << (n + i);
    }

    // memo[{先手の手札}][{後手の手札}] として、その手札から初めて先手が勝てるか
    // 遷移先は(先手が変わるので)swapされる
    let mut memo = vec![vec![2; 1 << s]; 1 << s];
    dfs(f, sec, &mut memo, &v);
    println!("{}", if memo[f][sec] == 0 { "Takahashi" } else { "Aoki" } );

}

