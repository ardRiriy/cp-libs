use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let a = ip.vector::<u64>(n);
    let s = a.iter().sum::<u64>();
    
    let mut dp = vec![0; n];
    
    for i in (1..n).rev() {
        dp[i-1] = dp[i-1].max(dp[i] + 1);

        if i > 1 {
            dp[i-2] = dp[i-2].max(dp[i] + a[i].min(i as u64));
        }

    }
    println!("{}", s-dp[0]);
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
