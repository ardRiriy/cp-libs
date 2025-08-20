use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let s = ip.next::<String>();
    let ans = match s.as_str() {
        "red" => "SSS",
        "blue" => "FFF",
        "green" => "MMM",
        _ => "Unknown",
    };
    println!("{}", ans);
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
