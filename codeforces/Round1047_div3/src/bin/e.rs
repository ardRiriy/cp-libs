use std::collections::BTreeMap;

use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n,k) = ip.pair();
    let mut a = ip.vector::<u64>(n);
    a.sort();
    
    if a[..n-1].iter().enumerate().all(|(i, ai)| i as u64 == *ai) && a[n-1] == n as u64 {
        a[n-1] = n as u64 -1;
    }

    let mut cnt = BTreeMap::new();
    let mut cur = 0;
    
    for ai in a.iter() {
        *cnt.entry(*ai).or_insert(0) += 1u64;
        if cur == *ai {
            cur += 1;
        }
    }

    let x = (0..=n+1).filter(|&i| !cnt.contains_key(&(i as u64))).min().unwrap() as u64;
    let y = x.min(
        (0..=n+1).filter(|&i| if let Some(val) = cnt.get(&(i as u64)) { *val != 1 } else { true }).min().unwrap() as u64
    );
    
    if k == 1 {
        let ans = a.iter()
            .map(|ai| {
                if *ai > x { x }
                else if cnt[ai] == 1 { *ai }
                else { x }
            })
            .sum::<u64>();
        println!("{}", ans);
    } else {
        if x == y {
            let ans = a.iter()
                .map(|ai| if *ai < y { *ai } else { if k % 2 == 0 { y + 1 } else { y }})
                .sum::<u64>();

            println!("{}", ans);
        } else {
            let ans = a.iter()
                .map(|ai| if *ai < y { *ai } else { if k % 2 == 0 { y } else { y + 1 }})
                .sum::<u64>();
            
            println!("{}", ans);
        }
        
    }

}

fn main() {
    static IS_MULTI_TEST_CASE :bool = true;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for i in 0..t {
        solve(&mut ip);
    }
}
