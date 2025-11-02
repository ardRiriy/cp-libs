use library::utils::{
    input::Input,
    iterlibs::{collect::CollectIter, dedup::RleItertor},
};

fn solve(ip: &mut Input) {
    let _n: usize = ip.next();
    let s = ip.chars();

    let rle = s.iter().rle().filter(|i| i.1 == &'1').collect_vec();

    println!(
        "{}",
        if rle.len() > 1 || rle.len() == 0 || !(rle[0].0 == 2 || rle[0].0 == 3) {
            "Yes"
        } else {
            "No"
        }
    );
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
