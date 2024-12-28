use proconio::input;
fn main() {
    input!{
        n: usize,
        p: [usize; n-1],
    }

    let mut target = n;
    let mut ans = 0;

    for (idx, x) in p.iter().enumerate().rev() {
        if idx + 2 == target {
            ans += 1;
            target = *x;
        }
    }
    println!("{ans}");
}
