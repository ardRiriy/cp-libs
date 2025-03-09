use cps::potentiality_unionfind::{DefaultPotentialMergeOp, PotentialityUnionfind};
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1, i64); m],
    }

    let mut uf :PotentialityUnionfind<i64, DefaultPotentialMergeOp> = PotentialityUnionfind::new(n, None);
    for &(u, v, w) in e.iter() {
        if let Err(_) = uf.merge(u, v, w) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

