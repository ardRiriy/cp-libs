use library::utils::{
    input::Input,
    iterlibs::{collect::CollectIter, strs::StrUtilIter},
};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let mut a = ip.vector::<u64>(n);
    let b = a.iter().map(|ai| ai % 2).collect_vec();
    if b.contains(&0) && b.contains(&1) {
        a.sort();
    }
    println!("{}", a.iter().join(" "));
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
