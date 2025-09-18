use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let v = ip.vector::<u64>(n);

    let mut first = None;
    let mut last = None;
    
    for i in 0..n {
        if v[i] == 1 {
            if let None = first {
                first = Some(i);
            }
            last = Some(i);
        }
    }
    if let Some(i) = first {
        let j = last.unwrap();
        println!("{}", j-i);
    } else {
        println!("0");
    }


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
