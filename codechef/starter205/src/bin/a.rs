use library::utils::{input::Input, iterlibs::collect::CollectIter};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let a = ip.vector::<i32>(n);
    
    println!("{}", if a.iter().copied().step_by(2).collect_vec().contains(&1) && a.iter().copied().skip(1).step_by(2).collect_vec().contains(&1) {
        "No"
    } else {
        "Yes"
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
