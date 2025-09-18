use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, q) = ip.pair::<usize, usize>();

    let mut a = ip.vector::<usize>(n);
    let mut b = ip.vector::<usize>(n);
    
    let mut ve = (0..n).map(|i| a[i].min(b[i])).collect::<Vec<_>>();
    let mut cur_sum = ve.iter().sum::<usize>();
    
    for _ in 0..q {
        let (c, x, v) = ip.triple::<char, usize, usize>();
        let x = x-1;
        
        cur_sum -= ve[x];
        
        if c == 'A' {
            a[x] = v;
        } else {
            b[x] = v;
        }
        ve[x] = a[x].min(b[x]);
        cur_sum += ve[x];
        println!("{}", cur_sum);
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
