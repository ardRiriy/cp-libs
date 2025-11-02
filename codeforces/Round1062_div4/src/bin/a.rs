use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let v = ip.vector::<i64>(4);
    println!(
        "{}",
        if v.iter().all(|vi| vi == &v[0]) {
            "YES"
        } else {
            "NO"
        }
    );
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
