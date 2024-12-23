#[allow(unused_imports)]
use itertools::Itertools;
use num_rational::Ratio;
use proconio::input;

fn main() {
    input!{
        n: usize,
        v: [(i64, i64); n],
    }
    
    let v = v.iter()
        .map(|&(u, v)| (Ratio::new(u, 1), Ratio::new(v, 1)))
        .collect_vec();

    let mut ans = Ratio::new(0, 1);
    let mut changed = false;
    for (&(x1, y1), &(x2, y2)) in v.iter().tuple_windows() {
        // y1 = x1 * a + b
        // y2 = x2 * a + b
        // y2 - (y1 * x2)/x1 = b - (x2 * b) / x1
        //                   = (1 - x2 / x1) * b
        
        let b = (y2 - (y1 * x2) / x1) / (Ratio::new(1, 1) - x2 / x1);
        if ans <= b {
            ans = b;
            changed = true;
        }
    }
    
    if changed {
        println!("{}", *ans.numer() as f64 / *ans.denom() as f64);
    } else {
        println!("-1");
    }
}

