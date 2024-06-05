use ac_library::Dsu;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        edge: [(Usize1, Usize1); m]
    }
    let mut dsu = edge.iter().fold(Dsu::new(n), |mut dsu, &(u, v)| { dsu.merge(u, v); dsu});
    println!("{}", (0..n).into_iter().fold(0, |acc, x| acc.max(dsu.size(x))));
}
