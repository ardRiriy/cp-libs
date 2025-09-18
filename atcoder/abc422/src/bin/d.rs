use library::utils::input::Input;

fn rec(n: u32, l: usize, ans: &mut Vec<u64>, val: u64) {
    if n == 0 {
        ans[l] += val;
        return;
    }
    
    let r = l + 2usize.pow(n-1);
    rec(n-1, l, ans, val/2);
    rec(n-1, r, ans, val-val/2);
} 

fn solve(ip: &mut Input) {
    let n = ip.next::<u32>();
    let m = 2usize.pow(n);
    let k = ip.next::<u64>();
    
    let mut ans = vec![0; m];
    rec(n, 0, &mut ans, k);
    println!("{}", if k % m as u64 == 0 { 0 } else { 1 });
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
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
