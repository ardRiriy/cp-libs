use std::collections::HashMap;

use library::{math::prime::prime_factorization, utils::{chlibs::ChLibs, input::Input}};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let a = ip.vector::<u64>(n);
    
    let mut ans = vec![];
    
    let mut map: HashMap<u64, (u64,u64,u64)> = HashMap::new();
    let mut cur = 0;
    
    for (i, &x) in a.iter().enumerate() {
        let pf = prime_factorization(x);
        
        for (p, c) in pf {
            if let Some((mx, mx_cnt, other)) = map.get_mut(&p) {
                if mx.chmax(c) {
                    *other += *mx_cnt;
                    *mx_cnt = 1;
                } else if mx == &c {
                    *mx_cnt += 1;
                } else {
                    *other += 1;
                }
                cur.chmax(*other);
            } else {
                map.insert(p, (c, 1, 0));
            }
        }
        ans.push(cur);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));

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
