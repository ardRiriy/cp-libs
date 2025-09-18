use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();

    let m = n * (n-1) / 2;
    let mut w = ip.vector::<u64>(m);
    w.sort();

    let mut ans = 0;
    let mut cur = 0;
    for i in 0..n-1 {
        ans += w[cur];
        //dbg!(cur, w[cur]);
        cur += 1;
        if i > 0 {
            let size = i;
            cur += size;
        }
    }

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
