use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let s = ip.chars();
    let c = s[0];

    if s.iter().filter(|si| si == &&c).count() == 1 {
        println!("{}", c);
    } else {
        println!("{}", s.iter().filter(|si| si != &&c).next().unwrap());
    }
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
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
