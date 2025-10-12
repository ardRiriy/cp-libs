use library::utils::input::Input;


fn solve(ip: &mut Input) {
    let n = ip.next::<i64>();
    println!("{}", (1..=n).map(|i| (-1i64).pow(i as u32) * i.pow(3)).sum::<i64>());
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
