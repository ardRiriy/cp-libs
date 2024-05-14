use proconio::{input, marker::Chars};

fn solve() {
    input! {
        s: Chars,
        t: Chars
    }
    let check = |c:char, d:char| {
        c == d || c == '?' || d == '?'
    };
    
    let diff = s.len() - t.len();
    let mut cnt = t.iter()
        .enumerate()
        .filter(|&(idx, &c)| {
            check(c, s[idx + diff])
        }).count();

    println!("{}", if cnt == t.len() {"Yes"} else {"No"});

    for i in 0..t.len() {
        if check(s[i + diff], t[i]) {
            cnt -= 1;
        }

        if check(s[i], t[i]) {
            cnt += 1;
        }
        
        println!("{}", if cnt == t.len() {"Yes"} else {"No"});
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
