use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let mut v = (0..n).map(|_| ip.pair::<u64,u64>()).collect::<Vec<_>>();
    v.sort();
    let ans = v.iter().map(|x| format!("{} {}", x.0, x.1)).collect::<Vec<_>>().join("\n");
    println!("{}", ans);
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
