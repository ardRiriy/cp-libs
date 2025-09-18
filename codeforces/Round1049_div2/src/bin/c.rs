use library::utils::{chlibs::ChLibs, consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let a = ip.vector::<i64>(n);
    
    let mut ans = if n % 2 == 0 { n as i64 - 2 } else { n as i64 - 1 };
    let mut k = -(INF as i64);
    let mut g = 1 - 2*a[0];
    
    for (i, &ai) in a.iter().enumerate().skip(1) {
        if i % 2 == 0 {
            ans.chmax(k - 2 * ai);
            g.chmax(-2 * ai);
        } else {
            ans.chmax(g + 2 * ai);
            k.chmax(2*ai);
        }
        g += 1;
        k += 1;
    }
    
    let cur = a.iter()
        .enumerate()
        .map(|(i, ai)| if i % 2 == 0 { *ai } else { - *ai })
        .sum::<i64>();
    println!("{}", cur + ans);

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
