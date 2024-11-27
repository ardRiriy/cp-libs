use cps::chlibs::ChLibs;
use proconio::input;
fn main() {
    input!{
        n: usize,
        x: usize,
        v: [(u64, u64); n],
    }

    const inf :u64 = 1<<60;
    let mut ans =  inf;

    let mut sum = 0;
    for (i, &(ai, bi)) in v.iter().enumerate() {
        if x < i {
            break;
        }
        ans.chmin((x - i) as u64 * bi + ai + sum);

        sum += ai + bi;
    }
    println!("{}", ans);
}

