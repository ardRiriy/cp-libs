use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }
    let mut uf = Dsu::new(n);
    let mut amari = vec![];
    for (i, (a, b)) in e.iter().enumerate() {
        if !uf.same(*a, *b) {
            uf.merge(*a, *b);
        } else {
            amari.push(i);
        }
    }

    // 0に集める
    let mut stk = vec![];
    for i in 0..n {
        if !uf.same(0, i) {
            stk.push(i);
        }
    }
    let mut ans = vec![];
    for i in amari {
        if stk.is_empty() {
            break;
        }
        if uf.same(0, e[i].0) {
            while let Some(x) = stk.pop() {
                if !uf.same(0, x) {
                    ans.push((i, e[i].0, x));
                    uf.merge(e[i].0, x);
                    break;
                }
            }
        } else {
            ans.push((i, e[i].0, 0));
            uf.merge(0, e[i].0);
        }
    }
    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|(i, a, b)| format!("{} {} {}", i + 1, a + 1, b + 1))
            .join("\n")
    );
}
