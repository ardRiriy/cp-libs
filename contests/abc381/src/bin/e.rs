use cps::veclibs::VecLibs;
use proconio::{input, marker::{Chars, Usize1}};


fn to_index(c: char) -> usize {
    match c {
        '/' => 0,
        '1' => 1,
        '2' => 2,
        _ => unreachable!(),
    }
}

fn judge(v: &Vec<Vec<usize>>, m: usize, l: usize, r: usize) -> bool {
    /* 長さ2*m+1の部分文字列があるか */
    let mut idx = l;
    if m != 0 {
        let k = v[to_index('1')].lower_bound(idx);
        if k + m - 1 >= v[to_index('1')].len() {
            return false;
        }
        idx = v[to_index('1')][k + m - 1];
    }

    {
        let k = v[to_index('/')].lower_bound(idx);
        if k >= v[to_index('/')].len() {
            return false;
        }
        idx = v[to_index('/')][k];
    }

    if m != 0 {
        let k = v[to_index('2')].lower_bound(idx);
        if k + m - 1 >= v[to_index('2')].len() {
            return false;
        }
        idx = v[to_index('2')][k + m - 1];
    }

    if idx <= r {
        true
    } else {
        false
    }

}

fn main() {
    input!{
        _n: usize,
        q: usize,
        s: Chars,
    }


    let mut v = vec![vec![]; 3];
    for (i, &c) in s.iter().enumerate() {
        v[to_index(c)].push(i);
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
        }

        let mut ok = !0;
        let mut ng = r-l+1;

        while ng.wrapping_sub(ok) > 1 {
            let m = ng.wrapping_add(ok) / 2;
            if judge(&v, m, l, r) {
                ok = m;
            } else {
                ng = m;
            }
        }
        if ok == !0 {
            println!("0");
        } else {
            println!("{}", ok * 2 + 1);
        }
    }
}
