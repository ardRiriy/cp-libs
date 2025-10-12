use std::collections::HashMap;

use library::utils::input::Input;

use ac_library::ModInt998244353 as Mint;

fn is_sub_seq(s: &Vec<char>, t: &Vec<char>) -> bool {
    let mut cur = 0;
    for ci in s.iter() {
        while cur < t.len() {
            if t[cur] == *ci {
                break
            }
            cur += 1;
        }
        if cur == t.len() {
            return false;
        }
        cur += 1;
    }
    true
}

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let t = ip.chars();
    
    let n1 = n/2;

    let n2 = n - n1;
    let mut dp2 = HashMap::new();
    dp2.insert(t.clone(), Mint::new(1));
    
    for i in 0..n2 {
        let mut ndp = HashMap::new();
        for (k, v) in dp2.iter() {
            for j in 0..k.len() {
                let mut nv = k.clone();
                nv.remove(j);
                if let Some(val) = ndp.get_mut(&nv) {
                    *val += v;
                } else {
                    ndp.insert(nv, *v);
                }
            }
        }
        dp2 = ndp;
    }

    dbg!(dp2.iter().next().unwrap());

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
