use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let v = ip.vector::<String>(n);
    let a = ip.pair::<usize, String>();
    
    println!("{}", if v[a.0-1] == a.1 { "Yes" } else { "No" }) ;

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
