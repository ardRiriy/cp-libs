use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (x, y) = ip.pair::<i64, i64>();
    let x = x-1;
    
    println!("{}", (x+y) % 12 + 1);

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
