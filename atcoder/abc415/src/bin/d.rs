use library::utils::{chlibs::ChLibs, consts::INF};
use proconio::input;

/*
N=1e18 大きい
M個の(ai,bi)の組

ai<=Nのうちai-biが最小のものを取り出す(累積minでok)

N<=aiである間、
ans <- ans+1
N <- N-(ai-bi)
を繰り返す

ここが重い=>O(1)でできる？
floor(N/(ai-bi))回でいいのか

操作できなくなるまでする
*/

fn main() {
    input!{
        n: u64,
        m: usize,
        mut ab: [(u64, u64); m],
    }

    ab.sort_by_key(|x| (x.0, x.0-x.1));

    let mut cmin = vec![];
    let mut cidx = vec![];
    let mut idx = !0;
    let mut cur = INF;
    for i in 0..m {
        if cur.chmin(ab[i].0 - ab[i].1) {
            idx = i;
        }
        cmin.push(cur);
        cidx.push(idx);
    }

    let mut ans = 0;
    let mut left = n;

    while left >= ab[0].0 {
        // ai <= leftであるような最大のai
        let p = ab.binary_search_by_key(&left, |x| x.0).unwrap_or_else(|x| x - 1);

        let p = cidx[p];

        let (a,b) = ab[p];
        
        let usable = left-a+1;
        let times = (usable + (a-b)-1) / (a-b);
        if times == 0 {
            break;
        }
        ans += times;
        left -= times * (a - b);
        //dbg!(p, a, b, usable, times, left);

    }

    println!("{}", ans);
}

