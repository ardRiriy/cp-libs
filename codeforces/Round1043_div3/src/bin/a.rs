use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let _n = ip.next::<usize>(); 
    let mut a = ip.next::<String>().chars().collect::<Vec<_>>();
    
    let _m = ip.next::<usize>();

    let c = ip.next::<String>().chars().collect::<Vec<_>>();
    let d = ip.next::<String>().chars().collect::<Vec<_>>();
    

    for (i, di) in d.iter().enumerate() {
        if di == &'D' {
            a.push(c[i]);
        } else {
            a.insert(0, c[i]);
        }
    }
    println!("{}", a.iter().collect::<String>());
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
