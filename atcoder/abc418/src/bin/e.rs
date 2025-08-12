use std::collections::BTreeMap;

use library::utils::input::Input;
use num_rational::Ratio;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let p = (0..n).map(|_| ip.pair::<i64, i64>()).collect::<Vec<_>>();

    let a = |x: (i64,i64), y: (i64,i64)| -> Option<Ratio<i64>> {
        let (x1, y1) = x;
        let (x2, y2) = y;
        if x2 == x1 {
            return None;
        } else {
            return Some(Ratio::new(y2-y1, x2-x1));
        }
    };
    
    let mid_p = |x: (i64,i64), y: (i64,i64)| -> (i64, i64) {
        (x.0 + y.0, x.1 + y.1)
    };
    
    let mut map = BTreeMap::new();
    let mut map2 = BTreeMap::new();
    let mut zero = 0u64;
    
    
    for i in 0..n {
        for j in i+1..n {
            if let Some(d) = a(p[i], p[j]) {
                *map.entry(d).or_insert(0) += 1u64;
            } else {
                zero += 1;
            }
            
            let p = mid_p(p[i], p[j]);
            *map2.entry(p).or_insert(0) += 1u64;
        }
    }
    
    let mut ans = map.iter().map(|(_, &v)| v * (v - 1) / 2).sum::<u64>();
    
    if zero >= 2 {
        ans += zero * (zero - 1) / 2;
    }
    
    let sub = map2.iter().map(|(_, &v)| v * (v - 1) / 2).sum::<u64>();
    
    println!("{}", ans - sub);
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
