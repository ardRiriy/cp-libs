use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut dsu = Dsu::new(n);
    let mut seen = vec![INF as usize; 1e6 as usize + 1];

    let sorted_a :Vec<u64> = a.iter().sorted().map(|x| *x).collect_vec();


    for (idx, &x) in sorted_a.iter().enumerate(){
        if seen[x as usize] != INF as usize{
            dsu.merge(idx, seen[x as usize]);
        }

        let mut k = x as usize;
        while k < 1e6 as usize + 1 {
            seen[k] = idx;
            k += x as usize;
        }
    }

    let mut ans = 0;
    let mut sum = sorted_a.iter().sum::<u64>();
    let mut cnt = vec![0; n];

    for (idx, &x) in sorted_a.iter().enumerate() {
        sum -= x;
        ans += sum / x;
        cnt[dsu.leader(idx)] += 1;
        ans -= (n + cnt[dsu.leader(idx)] - dsu.size(idx) - idx - 1) as u64;
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
