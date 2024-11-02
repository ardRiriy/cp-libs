use cps::chlibs::ChLibs;
use proconio::input;
fn main() {
    input!{
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; m],
    }

    a.sort_unstable();
    b.sort_unstable();
    let mut ans = 1 << 60;
    while a.len() != 0 && b.len() != 0 {
        let ai = a.pop().unwrap();
        let bi = b.pop().unwrap();

        ans.chmin(ai.abs_diff(bi));

        if ai >= bi {
            b.push(bi);
        } else {
            a.push(ai);
        }
    }
    println!("{}", ans);
}
