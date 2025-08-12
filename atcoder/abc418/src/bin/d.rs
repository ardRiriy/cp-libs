use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let s = ip.next::<String>().chars().collect::<Vec<_>>();
    
    // dp[i][j] := i番目までをevalしてjになるような者の個数
    
    let mut ans = 0u64;
    let mut dp = vec![0; 2];
    
    for i in 0..n {
        let mut ndp = vec![0; 2];
        
        if s[i] == '0' {
            ndp[1] += dp[0];
            ndp[0] += dp[1];
        } else {
            ndp[1] += dp[1];
            ndp[0] += dp[0];
        }
        
        ndp[if s[i] == '0' { 0 } else { 1 }] += 1;
        dp = ndp;
        ans += dp[1];
    }

    println!("{}", ans);
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
