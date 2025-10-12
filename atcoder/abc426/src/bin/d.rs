use library::utils::{
    input::Input,
    iterlibs::dedup::RleItertor,
};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let s = ip.chars();

    let zero_cnt = s.iter().filter(|ci| ci == &&'0').count();
    let one_cnt = n - zero_cnt;

    let ans = s
        .iter()
        .rle()
        .map(|(cnt, c)| (n - cnt) + if c == &'0' { zero_cnt } else { one_cnt } - cnt)
        .min()
        .unwrap();

    println!("{}", ans);
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
