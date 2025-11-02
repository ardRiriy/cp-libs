use library::utils::{
    input::Input,
    iterlibs::{collect::CollectIter, dedup::RleItertor},
};

fn solve(ip: &mut Input) {
    let _n: usize = ip.next();
    let s = ip.chars();
    let s = s.iter().rle().collect_vec();
    println!("{}", if s.len() >= 3 { "Alice" } else { "Bob" });
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = true;
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
