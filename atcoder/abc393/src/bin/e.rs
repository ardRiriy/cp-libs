use itertools::Itertools;
use proconio::input;

static M: usize = 1e6 as usize + 2;

fn divisors(has: &[bool]) -> Vec<Vec<usize>> {
    let mut res = vec![vec![]; M];
    let mut i = 2;
    while i < M {
        let mut j = i;
        while j < M {
            if has[j] {
                res[j].push(i);
            }
            j += i;
        }
        i += 1;
    }
    res
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let has = a.iter().fold(vec![false; M], |mut v, &ai| { v[ai] = true; v } );
    let v = divisors(&has);

    let mut cnt = vec![0; M];
    for &ai in a.iter() {
        for &vi in v[ai].iter() {
            cnt[vi] += 1;
        }
    }

    let mut ans = vec![1; n];
    'i: for (i, &ai) in a.iter().enumerate() {
        for &vi in v[ai].iter().rev() {
            if cnt[vi] >= k {
                ans[i] = vi;
                continue 'i;
            }
        }
    }
    println!("{}", ans.iter().join("\n"));
}
