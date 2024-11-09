use cps::veclibs::VecLibs;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        a: [Usize1; n],
        q: usize,
    }
    
    let mut poses = vec![vec![]; n];
    for (i, ai) in a.iter().enumerate() {
        poses[*ai].push(i);
    }
    
    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            x: Usize1,
        }
        
        let li = poses[x].lower_bound(l);
        let ri = poses[x].upper_bound(r);
        
        println!("{}", ri - li);
    }
}

