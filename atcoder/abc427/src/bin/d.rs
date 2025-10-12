use library::utils::input::Input;

fn dfs(p: usize, l: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<Vec<Option<bool>>>)  {
    // 先手勝ち → true
    if let Some(_val) = dp[p][l] {
        return;
    }
    
    let mut res = false;
    for &ni in g[p].iter() {
        dfs(ni, l-1, g, dp);
        if !dp[ni][l-1].unwrap() {
            res = true;
        }
    }

    dp[p][l] = Some(res);
}

fn solve(ip: &mut Input) {
    let (n,m,k) = ip.triple::<usize>();

    let s = ip.chars();
    let g = ip.graph(n, m, true);

    // dp[i][j] := 残りjターンで頂点iから始めたときに先手が勝てるか
    let mut dp = vec![vec![None; (2*k)+1]; n];
    for i in 0..n {
        dp[i][0] = Some(
            s[i] == 'A'
        );
    }

    let _ = dfs(0, 2*k, &g, &mut dp);
    println!("{}", if dp[0][2*k].unwrap() { "Alice" } else { "Bob" });
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
