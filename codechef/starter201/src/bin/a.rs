use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let a = ip.next::<String>();

    let a = a.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    
    let mut ans = a.iter().sum::<u32>();
    for i in 0..n-1 {
        ans += a[i] + a[i+1];
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

