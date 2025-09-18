use std::iter::repeat;

use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize, usize>();
    if n > 0 && m > 0 {
        let s1 = repeat("<").take(2 * n - 1).collect::<String>();
        let s2 = repeat("<>").take(m).collect::<String>();
        println!("{}{}", s1, s2);
    } else if n > 0 {
        let ans = repeat("<<").take(n-2).collect::<String>();
        println!("{ans}<=<");
    } else {
        let ans = repeat("><").take(m-2).collect::<String>();
        println!("{ans}>=>");
    }

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
