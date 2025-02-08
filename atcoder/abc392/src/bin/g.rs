use ac_library::convolution_i64;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }
    static N: usize = 1_000_000 + 2;
    let mut a = vec![0; N];
    for &si in &s {
        a[si] = 1;
    }
    let conv = convolution_i64(&a, &a);
    let ans = s.iter().map(|&si| (conv[2 * si] - 1) / 2).sum::<i64>();
    println!("{}", ans);
}
