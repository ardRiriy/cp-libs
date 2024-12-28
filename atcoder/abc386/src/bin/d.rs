#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        q: [(i64, i64, char); m],
    }

    let mut v = vec![(0, n as i64 + 1)];

    for &(i, j, _) in q.iter().filter(|&x| x.2 == 'W').sorted_unstable_by_key(|x| x.1) {
        if v.last().unwrap().1 > i {
            v.push((j, i));
        }
    }

    for &(i, j, _) in q.iter().filter(|&x| x.2 == 'B') {
        let mut ok = 0;
        let mut ng = v.len();
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if v[mid].0 <= j {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if v[ok].1 <= i {
            println!("No");
            return;
        }
    } 
    println!("Yes");
}
