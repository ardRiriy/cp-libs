use cps::chlibs::ChLibs;
use cps::consts::INF;
use proconio::input;

fn main() {
    input! {
        t: (i64, i64),
        bag: (i64, i64),
        target: (i64, i64),
    }

    let base = target.0.abs_diff(bag.0)
        + target.1.abs_diff(bag.1)
        + if target.0 == bag.0 || target.1 == bag.1 {
            0
        } else {
            2
        };

    let mut ans = INF;
    if target.0 != bag.0 {
        let to = if target.0 > bag.0 {
            (bag.0 - 1, bag.1)
        } else {
            (bag.0 + 1, bag.1)
        };
        let cost = t.0.abs_diff(to.0)
            + t.1.abs_diff(to.1)
            + if t.1 == bag.1 && !((t.0 <= to.0 && to.0 < bag.0) || (bag.0 < to.0 && to.0 <= t.0)) {
                2
            } else {
                0
            };
        ans.chmin(cost);
    }
    if target.1 != bag.1 {
        let to = if target.1 > bag.1 {
            (bag.0, bag.1 - 1)
        } else {
            (bag.0, bag.1 + 1)
        };
        let cost = t.0.abs_diff(to.0)
            + t.1.abs_diff(to.1)
            + if t.0 == bag.0 && !((t.1 <= to.1 && to.1 < bag.1) || (bag.1 < to.1 && to.1 <= t.1)) {
                2
            } else {
                0
            };
        ans.chmin(cost);
    }
    println!("{}", ans + base);
}
