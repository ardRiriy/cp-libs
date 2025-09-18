use library::utils::input::Input;

fn solve(ip: &mut Input) {
    
    let k = ip.next::<u64>();
    
    let mut l = 0;
    let mut r = 10u64;
    
    let mut seq = 0;
    
    let mut ans = 0u64;
    
    for i in 0.. {
        let len = r-l;
        
        if seq + len <= k {

        }


        
    }


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
