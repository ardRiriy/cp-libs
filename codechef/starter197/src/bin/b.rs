use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, k) = ip.pair::<usize, u64>();
    let s = ip.next::<String>();

    let ck = |x: u64| -> bool {
        let mut cnt = 0;
        let mut v = [0, 0];
        for c in s.chars() {
            if c=='0' {
                v[0] += 1;
            } else {
                v[1] = (v[1]+1).max(v[0]+1);
            }
            if v[0] >= x || v[1] >= x {
                cnt += 1;
                v[0] = 0;
                v[1] = 0;
            }
        }
        cnt >= k
    };

    let mut ok = 0;
    let mut ng = n as u64+1;
    while (ng-ok) > 1 {
        let mid = (ok+ng)>>1;
        if ck(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
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
