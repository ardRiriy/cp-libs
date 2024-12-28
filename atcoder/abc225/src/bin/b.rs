use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        e: [(Usize1, Usize1); n-1]
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    println!("{}", if g.iter().any(|v| v.len() == n-1) {
        "Yes"
    } else {
        "No"
    });
}

