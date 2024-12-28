use proconio::{input, marker::Chars};

fn solve() {
    input! {
        n: usize,
        ss: [Chars; n]
    }

    let mut g = vec![(0u64, vec![INF as usize; 26])];

    for s in ss.iter() {
        let mut idx = 0;
        for &c in s {
            if g[idx].1[c as usize - 'a' as usize] == INF as usize{
                g.push((0, vec![INF as usize; 26]));
                g[idx].1[c as usize - 'a' as usize] = g.len() - 1;
            }
            
            idx = g[idx].1[c as usize - 'a' as usize]; 
            g[idx].0 += 1;
        }
    }
    
    for s in ss.iter() {
        let mut ans = 0;
        let mut idx = 0;
        for (i, &c) in s.iter().enumerate() {
            idx = g[idx].1[c as usize - 'a' as usize];
            if g[idx].0 > 1 {
                ans.chmax(i + 1);
            } else {
                break;
            }
        }
        println!("{ans}");
    }


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
        if *self > elm {
                *self = elm;
                true
            } else { false }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
                *self = elm;
                true
            } else { false }
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
