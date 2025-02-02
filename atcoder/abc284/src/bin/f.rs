use ac_library::z_algorithm;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        t: Chars,
    }

    let a = t[0..n].iter().copied().collect_vec();
    let b = t[n..2*n].iter().rev().copied().collect_vec();
    let x = a.iter()
        .chain(&b)
        .copied()
        .collect::<String>();
    let y = b.iter()
        .chain(&a)
        .copied()
        .collect::<String>();

    let zx = z_algorithm(&x);
    let zy = z_algorithm(&y);

    for i in 0..=n {
        if (i==0 || zx[2*n-i] >= i) && (i==n || zy[n+i] >= n-i) {
            let a = &t[0..i];
            let b = &t[n+i..2*n];
            println!("{}{}", a.iter().join(""), b.iter().join(""));
            println!("{i}");
            return;
        }
    }
    println!("-1");
}

