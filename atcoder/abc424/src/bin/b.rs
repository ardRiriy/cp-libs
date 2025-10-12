use library::utils::{input::Input, iterlibs::strs::StrUtilIter};

fn solve(ip: &mut Input) {
    let (n, m, k) = ip.triple();
    let mut cnt = vec![0; n];
    let mut ans = Vec::new();
    for _ in 0..k {
        let (a, _b) = ip.pair::<usize>();
        cnt[a-1] += 1;
        if cnt[a-1] == m {
            ans.push(a);
        }
    }
    println!("{}", ans.iter().join(" "));
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
