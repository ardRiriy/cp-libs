use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (a, b, c) = ip.triple::<u64>();
    
    let mut ok = 0;
    let mut ng = a.min(c)+1;
    
    while ng-ok>1 {
        let mid = (ok+ng)>>1;
        
        let s = (a-mid) + b + (c-mid);
        if s >= mid {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
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
