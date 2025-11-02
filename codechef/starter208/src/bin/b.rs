use library::utils::{chlibs::ChLibs, consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let c = ip.next::<u64>();

    let b = ip.vector::<u64>(n);
    let d = ip.vector::<u64>(n);

    let mut cost_with_shift_times = vec![0u64; n + 1];

    for (i, di) in d.iter().enumerate() {
        let mut min_cost = INF;
        for j in 0..=n {
            min_cost.chmin(b[(i + n - j) % n] * *di);
            cost_with_shift_times[j] += min_cost;
        }
    }
    for i in 0..=n {
        cost_with_shift_times[i] += c * i as u64;
    }
    println!("{}", cost_with_shift_times.iter().min().unwrap());
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
