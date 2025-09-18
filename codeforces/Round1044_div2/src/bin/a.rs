use library::utils::{input::Input, iterlibs::dedup::RleItertor};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let mut a = ip.vector::<u64>(n);
    a.sort();

    println!("{}", if a.iter().rle().any(|(l, _)| l >= 2) {
        "Yes"
    } else {
        "No"
    });

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
