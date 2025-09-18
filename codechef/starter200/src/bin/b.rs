use std::collections::HashMap;

use library::utils::input::Input;


fn f0(x: usize, memo0: &mut HashMap<usize, usize>) -> usize {
    if x == 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }

    if let Some(val) = memo0.get(&x) {
        return *val;
    }
    
    let mut v = 1;
    while v * 2 <= x {
        v *= 2;
    }
    
    let res = v + f0(v-1, memo0) + f0(x-v, memo0);
    memo0.insert(x, res);
    return res;
}

fn f1(x: usize, memo0: &mut HashMap<usize, usize>, memo1: &mut HashMap<usize, usize>) -> usize {
    if x <= 1 {
        return 0;
    }
    
    if let Some(val) = memo1.get(&x) {
        return *val;
    }
    
    let mut v = 1;
    while v*2 <= x {
        v *= 2;
    }
    
    let res = v + f1(v-1, memo0, memo1) + f0(x-v, memo0) + if x==v { 1 } else { 0 };
    memo1.insert(x, res);
    return res;
}

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let ans = f1(n, &mut HashMap::new(), &mut HashMap::new());
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
