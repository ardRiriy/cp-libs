use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let a = ip.vector::<f64>(n);
    let b = ip.vector::<f64>(n);

    let mut cur = 0.;
    let mut b_csum = vec![0.];
    for bi in b.iter() {
        cur += *bi;
        b_csum.push(cur);
    }

    let nf = n as f64;
    let mut dp = vec![0.; n+1];
    let mut cur = 0.;

    for x in (0..n).rev() {
        dbg!(x);
        let xf = (x+1) as f64;
        let a_cost = a[x] + dp[x+1];
        let b_cost = b[x] + cur / (nf+1.) + b_csum[x+1]/(nf+1.-xf) + xf * cur / (nf+1.);
        dbg!(cur / (nf+1.));
        dbg!(b_csum[x+1] / (nf+1.-xf));
        dbg!(cur, a_cost, b_cost);

        dp[x] = a_cost.min(b_cost);
        cur += dp[x];
    }
    dbg!(&dp);
    println!("{}", dp[0]);
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
