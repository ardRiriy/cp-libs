use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize>();
    let v = (0..n).map(|_| ip.pair::<u64>()).collect::<Vec<_>>();

    let mut prev_t = 0;
    let mut prev_side = 0;
    
    let mut ans = 0;
    for (ai, bi) in v.iter() {
        let d = *ai - prev_t;
        if d % 2 == bi.abs_diff(prev_side) {
            ans += d;
        } else {
            ans += d - 1;
        }
        
        prev_t = *ai;
        prev_side = *bi;
    }

    let d = m as u64 - prev_t;
    ans += d;

    println!("{}", ans);
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
