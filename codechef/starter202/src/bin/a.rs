use library::utils::input::Input;

fn solve(ip: &mut Input) {

    let n = ip.next::<usize>();
    let a = ip.vector::<u64>(n);
    
    for i in 0..n-1 {
        if a[i] > a[i+1] {
            println!("{} {}", i+1, i+2);
            return;
        }
    }

    
    println!("-1");
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
