use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, m, k) = ip.triple::<usize>();

    let s = ip.chars();
    let g = ip.graph(n, m, true);

    // dp[i][j] := 残りjターンで頂点iから始めたときに先手が勝てるか
    let mut dp = vec![vec![false; 2 * k + 1]; n];
    for i in 0..n {
        dp[i][0] = s[i] == 'A';
    }

    for j in 1..=2 * k {
        for i in 0..n {
            dp[i][j] = g[i].iter().any(|ni| !dp[*ni][j - 1]);
        }
    }

    println!("{}", if dp[0][2 * k] { "Alice" } else { "Bob" });
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = true;
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
