use library::utils::input::Input;

fn f(x: u64) -> u64 {
    let mut res = 0;
    let mut k = x;
    while k > 0 {
        res += k % 10;
        k /= 10;
    }
    return res;
}

fn solve(ip: &mut Input) {
    let n = ip.next();
    
    let mut sum = 1;
    let mut cur = 1;
    for _ in 0..n {
        cur = sum;
        sum += f(cur);
    }
    println!("{}", cur);

}

fn main() {
    static IS_MULTI_TEST_CASE :bool = false;
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
