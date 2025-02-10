use cps::cumulative_sum::CumulativeSum;
use proconio::input;

fn find(left: usize, n: usize, csum: &CumulativeSum<u64>, key: u64) -> Option<usize> {
    // [left, res) = keyとなるようなresが存在するか

    let mut ng = left;
    let mut ok = n;

    while (ok-ng)>1 {
        let mid = (ok+ng)/2;
        if csum.get(left..mid) >= key {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    if csum.get(left..ok) == key {
        Some(ok)
    } else {
        None
    }
}

fn main() {
    input!{
        n: usize,
        p: u64,
        q: u64,
        r: u64,
        a: [u64; n],
    }

    let csum = CumulativeSum::new(&a);

    for x in 0..n {
        if let Some(y) = find(x, n, &csum, p) {
            if let Some(z) = find(y, n, &csum, q) {
                if let Some(w) = find(z, n, &csum, r) {
                    if x < y && y < z && z < w {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}

