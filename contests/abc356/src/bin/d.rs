use ac_library::ModInt998244353;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        m: usize,
    }

    println!("{}", dfs(2u64.pow(62u32) as usize, 62 ,n + 1, m));


}

fn dfs(t: usize, turn: usize ,n: usize, m: usize) -> ModInt998244353 {
    let mut res = ModInt998244353::new(0);


    if m >> turn & 1 == 1 {
        let nt = ModInt998244353::new(n / (2 * t)) * t;
        res += nt;
        res += if n % (2 * t) <= t {
            0
        } else {
            n % (2 * t) - t
        };
    }

    // if res != ModInt998244353::new(0) {
    //     println!("{t} {turn} {res}");
    // }

    if turn == 0 {
        return res;
    }

    res + dfs(t / 2, turn - 1 ,n, m)
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
