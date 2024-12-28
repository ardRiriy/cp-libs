use cps::consts::INF;
use cps::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let merge_op = |a: &Vec<isize>, b: &Vec<isize>| -> Vec<isize> {
        let mut aidx = 0;
        let mut bidx = 0;
        let mut res = vec![];
        while res.len() < 10{
            let ax = if aidx < 10 { a[aidx] } else { INF as isize };
            let bx = if bidx < 10 { b[bidx] } else { INF as isize };
            if ax >= bx {
                res.push(ax);
                aidx += 1;
            }  else {
                res.push(bx);
                bidx += 1;
            }
        }
        res
    };

    let mut uf = UnionFind::new(n, merge_op);

    for i in 0..n {
        let mut v = vec![-1; 10];
        v[0] = i as isize + 1;
        uf.insert_data(i, v);
    }

    for _ in 0..q {
        input! {
            t: u8,
            u: Usize1,
            v: Usize1,
        }
        match t {
            1 => {
                uf.merge(u, v);
            },
            2 => {
                let vec = uf.get_data(u).unwrap();
                println!("{}", vec[v]);
            },
            _ => { unreachable!(); }
        }
    }
}
