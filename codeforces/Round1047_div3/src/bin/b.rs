use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let p = ip.vector::<usize>(n);
    

    let q = p.iter()
        .map(|xi| n + 1  - *xi)
        .collect::<Vec<_>>();
    println!("{}", q.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(" "));
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
