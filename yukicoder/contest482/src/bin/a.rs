use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    if n % 2 == 0 {
        println!("No");
        return;
    }
    
    let base = (n+3)/2;
    let target = (base..base+n).collect::<Vec<_>>();
    let mut p = vec![0; n];
    
    let mut cur = 0;
    for i in 1..=n {
        p[cur] = i;
        cur += 2;
        cur %= n;
    }

    let mut q = vec![0; n];
    for (i, pi) in p.iter().enumerate() {
        q[i] = target[i] - *pi;
    }

    println!("Yes");
    println!("{}", p.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(" "));
    println!("{}", q.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(" "));

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
