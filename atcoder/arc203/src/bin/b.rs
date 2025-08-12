use library::utils::{input::Input, iterlibs::dedup::RleItertor};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let a = ip.vector::<u32>(n);
    let b = ip.vector::<u32>(n);
    let ac = a.iter().sum::<u32>();
    let bc = b.iter().sum::<u32>();
    
    if ac != bc {
        println!("No");
        return;
    }
    
    if a == b {
        println!("Yes");
        return;
    }
    
    println!("{}", if ac == 1 && (a.iter().rle().count() == 2 || b.iter().rle().count() == 2) {
        "No"
    } else {
        "Yes"
    });


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
