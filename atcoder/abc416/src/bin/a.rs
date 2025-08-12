use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, l, r) = ip.triple::<usize, usize, usize>();
    let l=l-1;
    let s = ip.next::<String>();
    println!("{}", if s[l..r].chars().all(|c| c=='o') { "Yes" } else { "No"} );

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
