use proconio::input;
fn main() {
    input!{
        a: u64,
        s: u64
    }

    if 2 * a > s {
        println!("No");
        return;
    }

    let mut left = s - 2 * a;

    let mut k = (0..60).fold(1u64, |acc, _| acc * 2);

    for i in (0..=60).rev() {
        if a >> i & 1 == 0 && left >= k {
            left -= k;
        }
        k /= 2;
    }

    if left == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
