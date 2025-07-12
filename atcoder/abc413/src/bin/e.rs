use itertools::Itertools;
#[allow(unused_imports)]
use proconio::input;

fn rec(l: usize, d: u32, a: &mut Vec<u64>) {
    if d==0 { return; }
    
    let nd = d-1;
    let next_length = 2usize.pow(nd);

    rec(l, nd, a);
    rec(l+next_length, nd, a);

    let mut is_same = true;
    let k = l+2usize.pow(d);
    for i in l..k {
        if a[i] < a[next_length+i] {
            break;
        } else if a[i] > a[next_length+i] {
            is_same = false;
            break;
        }
    }

    if !is_same {
        // 反転
        // for i in l..l+next_length {
        //     a.swap(i, k-i-1);
        // }
        a[l..l+next_length].reverse();
        a[(l+next_length)..k].reverse();
        a[l..k].reverse();
    }
}

fn solve() {
    input! {
        n: u32,
        mut a: [u64; 2usize.pow(n)]
    }

    rec(0, n, &mut a);
    println!("{}", a.iter().join(" "));
}

fn main() {
    input!{
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}


