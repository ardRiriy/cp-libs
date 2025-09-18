use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (x, n) = ip.pair::<i64>();
    println!("{}", if n%2==0 { 0 } else { x });

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
