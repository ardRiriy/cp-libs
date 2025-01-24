use std::mem::swap;

use itertools::Itertools;
use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn answer(n: usize, q: usize, a: &Vec<usize>, queries: &Vec<(usize, usize)>, indicates: &Vec<usize>) {
    let mut l = 0;
    let mut r = 0;
    let mut count = 0;
    let mut sum = vec![0; n];
    let mut ans = vec![0; q];

    for &i in indicates {
        let (li, ri) = queries[i];
        while l < li {
            sum[a[l]] -= 1;
            if sum[a[l]] == 0 {
                count -= 1;
            }
            l += 1;
        }
        while l > li {
            l -= 1;
            if sum[a[l]] == 0 {
                count += 1;
            }
            sum[a[l]] += 1;
        }

        while r < ri+1 {
            if sum[a[r]] == 0 {
                count += 1;
            }
            sum[a[r]] += 1;
            r += 1;
        }
        while r > ri+1 {
            r -= 1;
            sum[a[r]] -= 1;
            if sum[a[r]] == 0 {
                count -= 1;
            }
        }
        ans[i] = count;
    }
    println!("{}", ans.iter().join("\n"));
}

fn bucket(n: usize, q: usize, a: Vec<usize>, queries: Vec<(usize, usize)>) {
    let bucket_size = 2*n / q.sqrt();
    let mut indicates = (0..q).collect_vec();
    indicates.sort_unstable_by_key(|&i| (queries[i].0 / bucket_size, queries[i].1));
    answer(n, q, &a, &queries, &indicates);
}
    
#[allow(dead_code)]
fn hirbert(n: usize, q: usize, a: Vec<usize>, queries: Vec<(usize, usize)>) {
    // queriesをヒルベルト順にソート
    let mut indicates = (0..q).collect_vec();
    static LOG_N: usize = 20;
    static MAX_N: usize = 1 << LOG_N;
    let hirbert_index = |x: usize, y: usize| {
        let mut d = 0;
        let mut x = x;
        let mut y = y;
        let mut s = MAX_N >> 1;
        while s > 0 {
            let rx = if x & s > 0 { 1 } else { 0 };
            let ry = if y & s > 0 { 1 } else { 0 };
            d += s * s * ((3 * rx) ^ ry);
            if ry == 0 {
                if rx == 1 {
                    x = MAX_N - 1 - x;
                    y = MAX_N - 1 - y;
                }
                swap(&mut x, &mut y);
            }
            s >>= 1;
        }
        d
    };

    indicates.sort_unstable_by_key(|&i| hirbert_index(queries[i].0, queries[i].1));
    answer(n, q, &a, &queries, &indicates);
}

fn main() {
    input!{
        n: usize,
        q: usize,
        a: [Usize1; n],
        queries: [(Usize1, Usize1); q],
    }

    // hirbert(n, q, a, queries);
    bucket(n, q, a, queries);
}
