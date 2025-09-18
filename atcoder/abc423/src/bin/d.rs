use std::{cmp::Reverse, collections::BinaryHeap};

use library::utils::{chlibs::ChLibs, input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let k = ip.next();

    let mut sum = 0;
    let mut pq = BinaryHeap::new();
    let mut last = 0;
    
    for _ in 0..n {
        let (a, b, c) = ip.triple::<u64>();
        let mut ans = a.max(last);
        while sum + c > k {
            let Reverse((t, cnt)) = pq.pop().unwrap();
            ans.chmax(t);
            sum -= cnt;
        }
        sum += c;
        println!("{}", ans);
        last = ans;
        pq.push(Reverse((ans+b, c)));
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
