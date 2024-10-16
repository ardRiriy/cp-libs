use std::{collections::BinaryHeap, process::Output};

use ac_library::Dsu;
use itertools::{iproduct, Itertools};
use num::Integer;
use proconio::input;
/*
最大全域木を求める問題に帰着することができる。

より正確には、この問題の操作は(操作に対応する適切な木を用意することで)以下のように言い換えることができる。

ある全域木Gに対して、葉であるような頂点iを選択し、その親の頂点をjとする。
i, j間の重みを点数として得たあと、頂点iを削除する。

最終的に得られる得点は全域木Gの重みの総和であり、重みの総和を最大化するような全域木こそが最大全域木である。
*/

fn main() {
    input!{
        n: usize,
        m: u64,
        a: [u64; n]
    }

    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in i+1..n {
            let w = (modpow(a[i], a[j], m) + modpow(a[j], a[i], m)) % m;
            g[i].push((j, w));
            g[j].push((i, w));
        }
    }

    let mut pq = BinaryHeap::new();
    let mut ans = 0;
    let mut seen = vec![false; n];
    pq.push((0, 0));
    while let Some((w, p)) = pq.pop() {
        if seen[p] {
            continue;
        } else {
            seen[p] = true;
            ans += w;
        }
        for &(ni, w) in &g[p] {
            if !seen[ni] {
                pq.push((w, ni));
            }           
        }
    }
    println!("{ans}");
}

// a^n % p を計算
fn modpow<T>(a: T, n: T, p: T) -> T 
where T: Integer + From<u64> + std::ops::Shr<usize, Output = T> + std::ops::BitAnd<Output = T> + Copy
{
    let mut x:T = a;
    let mut res :T = T::from(1);
    let one = T::from(1);
    for i in 0..64 {
        if (n >> i) & one == one {
            res = (res * x) % p;
        }
        x = (x * x) % p;
    }
    res % p
}
