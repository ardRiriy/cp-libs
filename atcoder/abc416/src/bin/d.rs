use std::collections::BTreeMap;

use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize, u64>();
    let mut a = ip.vector::<u64>(n);
    let b = ip.vector::<u64>(n);

    let mut ans = a.iter().sum::<u64>() + b.iter().sum::<u64>();

    let mut map = BTreeMap::new();
    for bi in b {
        *map.entry(bi).or_insert(0u64) += 1;
    }

    a.sort();
    
    for ai in a.iter().rev() {
        let nd = m-*ai;
        let mut del = None;
        if let Some((k,v)) = map.range_mut(nd..).next() {
            ans -= m;
            *v -= 1;
            if *v==0 {
                del = Some(*k);
            }
        } else {
            break;
        }

        if let Some(key) = del {
            map.remove(&key);
        }
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
