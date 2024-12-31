use std::collections::VecDeque;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        a: [[Usize1; n-1]; n],
    }

    let mut day = vec![0u64; n];
    let mut indicates = vec![0; n];

    let mut que = VecDeque::new();
    for i in 0..n {
        que.push_back(i);
    }

    while let Some(i) = que.pop_front() {
        if indicates[i] == n-1 {
            continue;
        }
        let j = a[i][indicates[i]];
        if indicates[j] == n-1 {
            continue;
        }
        if i == a[j][indicates[j]] {
            let d = day[i].max(day[j]) + 1;
            day[i] = d;
            day[j] = d;
            indicates[i] += 1;
            indicates[j] += 1;
            if indicates[i] != n-1 {
                que.push_back(i);
            }
            if indicates[j] != n-1 {
                que.push_back(j);
            }
        } 
    }
    if indicates.iter().all(|x| *x == n-1) {
        println!("{}", day.iter().max().unwrap());
    } else {
        println!("-1");
    }
}
