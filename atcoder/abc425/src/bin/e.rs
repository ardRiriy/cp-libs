use library::utils::input::Input;

fn main() {
    static IS_MULTI_TEST_CASE :bool = true; 
    static M:usize = 5000;
    let mut ip = Input::new();
    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    let m = ip.next::<u64>();
    let mut ncr = vec![vec![0; M+1]; M+1];
    for i in 0..=M {
        ncr[i][0] = 1;
        for j in 1..=i {
            ncr[i][j] = (ncr[i-1][j-1] + ncr[i-1][j]) % m;
        }
    }
    

    for _ in 0..t {
        let n = ip.next();
        let c = ip.vector::<usize>(n);
        
        let mut left = c.iter().sum::<usize>();
        let mut ans = 1;
        for ci in c.iter() {
            ans *= ncr[left][*ci];
            ans %= m;
            left -= *ci;
        }
        println!("{}", ans);
    }
}
