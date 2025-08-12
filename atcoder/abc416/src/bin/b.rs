use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let s = ip.next::<String>();
    let mut s = s.replace("#.", "#o").chars().collect::<Vec<_>>();
    if s[0] == '.' {
        s[0] = 'o';
    }
    println!("{}", s.iter().collect::<String>());
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
