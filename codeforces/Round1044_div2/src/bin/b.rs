use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let mut a = ip.vector::<u64>(n);
    a.sort();
    a.reverse();
    let mut ans = 0;
    for i in (0..n).step_by(2) {
        ans += a[i];
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
