use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (a, b) = ip.pair::<u64>();
    let (c, d) = ip.pair();
    println!("{}", if a <= c && b > d { "Yes" } else { "No" });
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
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
