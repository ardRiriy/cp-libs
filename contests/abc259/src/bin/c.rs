use proconio::input;
use proconio::marker::Chars;
fn solve() {
    input! {
        s: Chars,
        t: Chars
    }

    let sv = s.iter().fold(
        vec![],
        |mut res:Vec<(char, u64)>, &c| {
            if res.len() == 0 || res[res.len() - 1].0 != c {
                res.push((c, 1u64));
            } else {
                let n = res.len();
                res[n - 1].1 += 1;
            }
            res
        }
    );

    let tv = t.iter().fold(
        vec![],
        |mut res: Vec<(char, u64)>, &c| {
            if res.len() == 0 || res[res.len() - 1].0 != c {
                res.push((c, 1u64));
            } else {
                let n = res.len();
                res[n - 1].1 += 1;
            }
            res
        });

    if sv.len() != tv.len() { println!("No"); return; }

    for i in 0..sv.len() {
        if sv[i].0 != tv[i].0 { println!("No"); return; }
        if sv[i].1 > tv[i].1 || (sv[i].1 == 1 && tv[i].1 != 1){
            println!("No"); return;
        }
    }
    println!("Yes");
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
