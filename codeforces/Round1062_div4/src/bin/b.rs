use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let _ = ip.next::<i32>();
    let mut s = ip.chars();
    let mut t = ip.chars();
    s.sort();
    t.sort();
    println!("{}", if s == t { "YES" } else { "NO" });
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
