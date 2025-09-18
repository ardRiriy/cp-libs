use library::utils::{chlibs::ChLibs, consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let _n = ip.next::<usize>();
    let s = ip.next::<String>().chars().collect::<Vec<_>>();

    let mut ans = INF;
    
    // abab
    for start in 0..=1 {
        let mut cnt = 0;
        let mut cur_pos = start;
        for (i, c) in s.iter().enumerate() {
            if c == &'B' { continue; }
            cnt += i.abs_diff(cur_pos);
            cur_pos += 2;
        }
    
        ans.chmin(cnt as u64);
    }

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
