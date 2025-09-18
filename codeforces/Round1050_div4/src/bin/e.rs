use std::collections::BTreeMap;

use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, k) = ip.pair();
    let a = ip.vector::<u64>(n);
    
    let mut map = BTreeMap::new();
    for &x in a.iter() {
        *map.entry(x).or_insert(0) += 1;
    }
    
    let mut uppers = BTreeMap::new();
    
    for (key, &v) in map.iter() {
        if v % k != 0 {
            println!("0");
            return;
        }
        uppers.insert(*key, v / k);
    }
    
    let mut ans = 0u64;
    let mut cnts = BTreeMap::new();
    
    let mut l = 0;
    for (r, &v) in a.iter().enumerate() {
        *cnts.entry(v).or_insert(0) += 1;
        
        while cnts[&v] > uppers[&v] {
            *cnts.entry(a[l]).or_insert(0) -= 1;
            l += 1;
        }
        
        ans += (r - l + 1) as u64;
    }

    println!("{}", ans);
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
