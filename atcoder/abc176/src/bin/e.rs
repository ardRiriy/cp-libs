use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        h: usize,
        w: usize,
        m: usize,
        p: [(Usize1, Usize1); m],
    }
    let mut hv = vec![None; h];
    let mut wv = vec![None; w];
    let mut uf = Dsu::new(m);
    for (i, &(hi, wi)) in p.iter().enumerate() {
        if let Some(j) = hv[hi] {
            uf.merge(i, j);
        } else {
            hv[hi] = Some(i);
        }
        if let Some(j) = wv[wi] {
            uf.merge(i, j);
        } else {
            wv[wi] = Some(i);
        }
    }

    let ans = (0..m)
        .filter_map(|i| if uf.leader(i) == i { Some(uf.size(i)) } else { None })
        .max()
        .unwrap();

    println!("{}", ans);
}

