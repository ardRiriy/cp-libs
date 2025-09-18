use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let s = ip.chars();
    let mut t = s.clone();
    t.sort();
    let cnt = (0..n).filter(|&i| s[i] != t[i]).count();
    println!("{}", cnt/2);
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
