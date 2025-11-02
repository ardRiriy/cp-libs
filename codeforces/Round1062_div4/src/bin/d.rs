use library::{math::prime::create_primes, utils::input::Input};

fn solve(ip: &mut Input, primes: &Vec<u64>) {
    let n = ip.next();
    let a = ip.vector::<u64>(n);
    let ans = a
        .iter()
        .map(|ai| primes.iter().filter(|&pi| ai % pi != 0).min().unwrap())
        .min()
        .unwrap();
    println!("{}", ans);
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = true;
    let mut ip = Input::new();

    let primes = create_primes(60);
    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip, &primes);
    }
}
