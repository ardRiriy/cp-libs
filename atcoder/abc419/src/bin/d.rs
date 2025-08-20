use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize,usize>();
    let s = ip.next::<String>().chars().collect::<Vec<_>>();
    let t = ip.next::<String>().chars().collect::<Vec<_>>();

    let mut imos = vec![0i64; n+1];
    
    for _ in 0..m {
        let (l, r) = ip.pair::<usize, usize>();
        imos[l-1] += 1;
        imos[r] -= 1;
    }

    let mut ans = vec![];
    let mut cur = 0;
    
    for (i, imos_i) in imos[..n].iter().enumerate() {
        cur += *imos_i;
        if cur % 2 == 0 {
            ans.push(s[i]);
        } else {
            ans.push(t[i]);
        }
    }
    println!("{}", ans.iter().collect::<String>());

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
