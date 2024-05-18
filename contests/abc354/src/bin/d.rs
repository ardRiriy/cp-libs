use proconio::input;

#[allow(non_snake_case)]
fn solve() {
    input! {
        A: isize,
        B: isize,
        C: isize,
        D: isize,
    }

    let a = if (A + 1e9 as isize) % 4 == 0 {
        0
    } else {
        4 - (A + 1e9 as isize) % 4
    };
    let b = if (B + 1e9 as isize) % 2 == 0 {
        0
    } else {
        2 - (B + 1e9 as isize) % 2
    };
    let c = (C + 1e9 as isize) % 4;
    let d = (D + 1e9 as isize) % 2;

    // 中央
    // println!("{} {} {} {}", a, b, c, d);
    let mut ans = (((C - c) - (A + a)) / 4) * (((D - d) - (B + b)) / 2) * 8;

    // 下
    // println!("{}", ans);
    ans += if b == 1 {
        (((C - c) - (A + a)) / 4) * 4
            + match a {
                1 => 0,
                2 => 1,
                3 => 3,
                _ => 0,
            }
            + match c {
                1 => 1,
                2 => 3,
                3 => 4,
                _ => 0,
            }
    } else {
        0
    };

    // println!("{}", ans);
    // 上
    ans += if d == 1 {
        (((C - c) - (A + a)) / 4) * 4
            + match a {
                1 => 1,
                2 => 1,
                3 => 2,
                _ => 0,
            }
            + match c {
                1 => 2,
                2 => 3,
                3 => 3,
                _ => 0,
            }
    } else {
        0
    };
    // println!("{}", ans);

    let tate = ((D - d) - (B + b)) / 2;
    // println!("tate: {}", tate);
    // 右
    ans += tate
        * match c {
            1 => 3,
            2 => 6,
            3 => 7,
            _ => 0,
        };

    // println!("{}", ans);
    ans += tate
        * match a {
            1 => 1,
            2 => 2,
            3 => 5,
            _ => 0,
        };

    println!("{}", ans);
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
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
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
