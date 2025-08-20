use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let v = (0..n).map(|_|ip.pair::<u64,u64>()).collect::<Vec<_>>();
    
    let x = v.iter().map(|v| v.0).collect::<Vec<_>>();
    let y = v.iter().map(|v| v.1).collect::<Vec<_>>();
    
    let px = (*x.iter().max().unwrap() + *x.iter().min().unwrap()) / 2;
    let py = (*y.iter().max().unwrap() + *y.iter().min().unwrap()) / 2;
    
    let ans = v.iter()
        .map(|v| v.0.abs_diff(px).max(v.1.abs_diff(py)))
        .max()
        .unwrap();
    
    println!("{}", ans);

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
