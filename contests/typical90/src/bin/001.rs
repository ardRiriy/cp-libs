use proconio::input;

fn solve() {
    input! {
        n: usize,
        l: u64,
        k: usize,
        a: [u64; n]
    }

    // i cm以上の羊羹に分割したとき、K+1ピースに分割できるか？
    // ↑のiを二分探索
    let mut ok = 1;
    let mut ng = 1e9 as u64 + 1;

    let judge = |size: u64| -> bool {
        let mut cnt = 0;
        let mut last_cut = 0;

        for &x in a.iter() {
            if x - last_cut >= size {
                cnt += 1;
                last_cut = x;
            }
        }

        if l - last_cut >= size {
            cnt += 1;
        }

        cnt > k
    };


    while ng - ok > 1 {
        // println!("{} {}", ok, ng);
        let mid = (ok + ng) / 2;

        if judge(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    eprintln!("{} {}", ok, ng);
    println!("{}", ok);
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
