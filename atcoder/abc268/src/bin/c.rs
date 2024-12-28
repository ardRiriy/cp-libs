use cps::chlibs::ChLibs;
use proconio::input;
fn main() {
    input!{
        n: usize,
        p: [usize; n],
    }

    let mut cnt = vec![0; n];
    for (i, &pi) in p.iter().enumerate() {
        if pi <= i {
            cnt[i - pi] += 1;
        } else {
            cnt[n - (pi - i)] += 1;
        }
    }
    
    let mut ans = 0;
    for i in 0..n {
        let mut t = 0;
        for j in 0..3 {
            t += cnt[(i + j) % n];
        }
        ans.chmax(t);
    }
    println!("{ans}");
}


