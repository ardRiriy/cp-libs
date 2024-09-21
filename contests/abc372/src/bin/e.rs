use std::collections::BTreeSet;

use ac_library::Dsu;
use cps::consts::INF;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut uf = Dsu::new(n);

    let mut ans = vec![vec![-1i64; 10]; n];
    for i in 0..n {
        ans[i][0] = 1 + i as i64;
    }

    for _ in 0..q {
        input! {
            t: u32,
            u: Usize1,
            v: Usize1,
        }

        match t {
            1 => {
                if !uf.same(u, v) {

                    let uidx = uf.leader(u);
                    let vidx = uf.leader(v);
                    uf.merge(u, v);
                    let mut nv = vec![];
                    let mut i = 0;
                    let mut j = 0;
                    while nv.len() < 10 {
                        let uval = ans[uidx].get(i).copied().unwrap_or(-1);
                        let vval = ans[vidx].get(j).copied().unwrap_or(-1);

                        if uval == -1 && vval == -1 {
                            break;
                        } else if uval >= vval {
                            if uval != -1 {
                                nv.push(uval);
                            }
                            i += 1;
                        } else {
                            if vval != -1 {
                                nv.push(vval);
                            }
                            j += 1;
                        }
                    }

                    while nv.len() < 10 {
                        nv.push(-1);
                    }
                    ans[uf.leader(u)] = nv;
                }
            },
            2 => {
                println!("{}", ans[uf.leader(u)][v]);
            }
            _ => { unreachable!(); }
        }
    }
}
