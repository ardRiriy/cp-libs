use proconio::input;
fn main() {
    input!{
        n: usize,
        mut a: [u64; n],
        mut b: [u64; n-1],
    }

    let mut checked = vec![false; n];
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();
    let mut ai = 0;
    for &x in &b {
        while ai < n {
            if x >= a[ai] {
                checked[ai] = true;
                ai += 1;
                break;
            }
            ai += 1;
        }
    }
    if checked.iter().filter(|b| **b).count() == n-1 {
        let pos = checked.iter().position(|b|!*b).unwrap();
        println!("{}", a[pos]);
    } else {
        println!("-1");
    }
}

