use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let s = ip.next::<u64>();
    let (a, b) = ip.pair::<u64>();
    let x = ip.next::<u64>();

    let left = x % (a + b);
    let ans = s * a * (x / (a + b)) + s * a.min(left);
    println!("{}", ans);
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
