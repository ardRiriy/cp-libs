use proconio::{input};
fn main() {
    input!{
        mut v: u64,
        x: [u64; 3],
    }
    let a = &['F', 'M', 'T'];

    'l: loop {
        for (idx, &k) in x.iter().enumerate() {
            if v < k {
                println!("{}", a[idx]);
                break 'l;
            } else {
                v -= k;
            }
        }
    }
}
