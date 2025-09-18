use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize>();
    let _ = ip.pair::<usize>();
    
    for _ in 0..n+m {
        ip.next::<String>();
    }

    println!("{}", n+m);
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
