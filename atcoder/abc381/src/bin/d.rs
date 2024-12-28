use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
#[allow(unused_imports)]
use cps::dbg;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        a: [Usize1; n],
    }

    let v = a.iter()
        .copied()
        .dedup_by_with_count(|x, y| x == y)
        .collect_vec();

    let mut l = 0;
    let mut r = 0;

    let mut cnt = 0u64;
    let mut seen = vec![false; n];
    let mut ans = 0u64;

    while r < v.len() {
        if v[r].0 == 2 {
            if seen[v[r].1] {
                for i in l..r {
                    if v[i].1 == v[r].1 {
                        l = i + 1;
                        break;
                    } else {
                        seen[v[i].1] = false;
                        cnt -= 1;
                    }
                }
            } else {
                seen[v[r].1] = true;
                cnt += 1;
                ans.chmax(cnt * 2);
            }
            r += 1;
        } else if v[r].0 == 1 {
            for i in l..r {
                seen[v[i].1] = false;
            }
            cnt = 0;
            r += 1;
            l = r;
        } else {
            if !seen[v[r].1] {
                ans.chmax((cnt + 1) * 2);
            }
            for i in l..r {
                seen[v[i].1] = false;
                if v[i].1 == v[r].1 {
                    ans.chmax(cnt * 2);
                } else {
                    cnt -= 1;
                }
            }
            l = r;
            seen[v[r].1] = true;
            cnt = 1;
            r += 1;
        }
    }
    ans.chmax(cnt * 2);
    println!("{ans}");
}
