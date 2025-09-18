use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, q) = ip.pair::<usize, usize>();
    let a = ip.vector::<u64>(n);
    
    let mut v = [a[0], a[0]];
    
    for (i, &ai) in a[1..].iter().enumerate() {
        if v[0] < v[1] {
            v[1 - i % 2] = ai;
        } else {
            v[i%2] = ai;
        }
    }
    println!("{}", v[0].min(v[1]));
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
