use library::utils::{input::Input, iterlibs::collect::CollectIter};

fn solve(ip: &mut Input) {
    let v = ["Ocelot", "Serval", "Lynx"]
        .iter()
        .map(|s| s.to_string())
        .collect_vec();
    let (a, b) = ip.pair::<String>();

    let ai = v.iter().position(|x| x == &a).unwrap();
    let bi = v.iter().position(|x| x == &b).unwrap();

    println!("{}", if ai >= bi { "Yes" } else { "No" });
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
