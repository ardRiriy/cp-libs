use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let m = ip.next::<u64>();
    let a = ip.vector::<u64>(n);
    let s = a.iter().sum::<u64>();
    println!(
        "{}",
        if a.iter().any(|ai| s - *ai == m) {
            "Yes"
        } else {
            "No"
        }
    );
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
