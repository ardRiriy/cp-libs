use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let mut a = ip.vector::<u64>(n);
    a.sort();
    let ans = if a[n - 1] - a[0] == 1 {
        a[0] - 1
    } else {
        a[n - 1] - 1
    };
    println!("{}", ans);
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
