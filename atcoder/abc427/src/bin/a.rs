use library::utils::{input::Input, iterlibs::strs::StrUtilIter};

fn solve(ip: &mut Input) {
    let mut s = ip.chars();
    s.remove(s.len() / 2);

    println!("{}", s.iter().join(""));
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
