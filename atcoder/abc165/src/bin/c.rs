use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::Usize1;

fn rec(v: &mut Vec<i32>, m: usize, n: usize, i: usize, queries: &Vec<(usize,usize,i32,i32)>) -> i32 {
    if i==m {
        let l = v.len();
        for _ in 0..n-l {
            v.push(i as i32);
        }

        let res = queries.iter()
            .map(|&(i,j,k, d)| if v[j] - v[i] == k { d } else { 0 })
            .sum::<i32>();

        for _ in 0..n-l {
            v.pop();
        }

        return res;
    }

    // 何もしない
    let mut res = rec(v,m,n,i+1,queries);
    let l = v.len();
    for _ in 0..n-l {
        v.push(i as i32);
        res.chmax(rec(v,m,n,i+1,queries));
    }
    
    for _ in 0..n-l {
        v.pop();
    }
    return res;
}

fn main() {
    input!{
        n: usize,
        m: usize,
        q: usize,
        queries: [(Usize1, Usize1, i32, i32); q],
    }

    println!("{}", rec(&mut vec![], m, n, 1, &queries));
}

