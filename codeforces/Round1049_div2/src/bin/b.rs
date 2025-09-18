use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let x = ip.next::<u64>();
    println!("{}", x*2);
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
