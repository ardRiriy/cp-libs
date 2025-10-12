use std::collections::BTreeSet;

use library::utils::{input::Input, iterlibs::strs::StrUtilIter};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let mut a = ip.vector::<i64>(n);
    let mut pool = BTreeSet::from_iter((1..=n).into_iter());
    
    for &ai in a.iter() {
        if ai != -1 {
            pool.remove(&(ai as usize));
        }
    }
    
    for ai in a.iter_mut() {
        if ai != &-1 {
            continue;
        }
        *ai = pool.pop_first().unwrap() as i64;
    }
    
    if pool.len() == 0 {
        println!("Yes\n{}", a.iter().join(" "));
    } else {
        println!("No");
    }

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
