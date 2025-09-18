use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let a = ip.vector::<u64>(n);
    if a.iter().all(|&x| x%2==0) {
        println!("0");
        return;
    }

    let mut odd = a.iter().copied().filter(|&x| x % 2 == 1).collect::<Vec<_>>();
    odd.sort();
    
    let ans = a.iter().filter(|&x| x%2==0 ).sum::<u64>() + odd[odd.len()/2..].iter().sum::<u64>();
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
