use library::utils::input::Input;

fn f(x: u64) -> u64 {
    if x == 0 {
        return 3;
    }
    3u64.pow(x as u32 +1) + x * 3u64.pow(x as u32-1)
}

fn solve(ip: &mut Input) {
    let n = ip.next::<u64>();
    
    let mut left = n;
    let mut i = 0;
    let mut ans = 0;
    
    while left > 0 {
        ans += left % 3 * f(i);
        
        i += 1;
        left /= 3;
    }

    println!("{}", ans);

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
