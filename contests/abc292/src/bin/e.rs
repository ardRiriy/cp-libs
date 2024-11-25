use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| { g[u].push(v); g });
    let mut ans = 0;

    for i in 0..n {
        let mut seen = vec![false; n];
        seen[i] = true;
        let mut stk = vec![i];

        while let Some(p) = stk.pop() {
            for &ni in g[p].iter() {
                if !seen[ni] {
                    seen[ni] = true;
                    stk.push(ni);
                }
            }
        }

        let count = seen.iter().filter(|b| **b).count() - 1;
        ans += count - g[i].len();
    }
    println!("{ans}");
}

