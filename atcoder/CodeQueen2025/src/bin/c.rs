use ac_library::ModInt998244353 as Mint;
use library::utils::input::Input;
fn solve(ip: &mut Input) {
    let (n, k) = ip.pair::<usize,usize>();
    
    let mut ans = Mint::new(1);
    for i in 1..=n {
        ans *= (k+1).min(n+1-i);
    }
    println!("{}",ans); 
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
