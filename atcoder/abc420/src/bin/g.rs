use library::utils::input::Input;

fn exp() {
    for i in 0..100 {
        println!("{i}={}", i * (i+1));
    }
}

fn solve(ip: &mut Input) {
    exp();

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
