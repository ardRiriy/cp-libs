use proconio::input;

fn solve() {
    input! {
        n: u64
    }

    let mut i = 1;
    let mut ans = 0;
    while i <= n {
        let k = n / i;

        // println("{}", k);
        let mut ok = i as i64;
        let mut ng = n as i64 + 1;

        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if n / mid as u64 == k {
                ok = mid;
            } else {
                ng = mid;
            }
            
            // println!("# {} {}", ok, ng);
        }
        
        // println!("{} {}", i, ok);
        ans += (ok as u64 + 1 - i) * k;
        i = ok as u64 + 1;
    }
    println!("{ans}");

}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/


static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        return
            if *self > elm {
                *self = elm;
                true
            } else { false };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return
            if *self < elm {
                *self = elm;
                true
            } else { false };
    }
}


fn main() {
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
