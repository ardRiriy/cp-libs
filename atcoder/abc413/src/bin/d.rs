use itertools::Itertools;
use num_rational::Ratio;
#[allow(unused_imports)]
use proconio::input;


fn solve() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let ca = a.iter().sorted().copied().dedup_with_count().collect_vec();
    if ca.len() == 1 {
        println!("Yes");
        return;
    } else if ca.len() == 2 && ca[0].1 == -ca[1].1 {
        if ca[0].0.abs_diff(ca[1].0) <= 1 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }
    
    let a = a.iter()
        .sorted_by_key(|x| x.abs())
        .map(|&x| Ratio::new(x,1))
        .collect_vec();

    
    let r = a[1] / a[0];
    let mut cur = a[0];
    for i in 1..n {{
        cur *= r;
        if cur != a[i] {
            println!("No");
            return;
        }

    }}

    println!("Yes");
}

fn main() {
    input!{
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

