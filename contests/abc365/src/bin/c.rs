use proconio::input;
fn main() {
    input!{
        n: usize,
        m: u64,
        mut a: [u64; n]
    }

    a.sort();
    let mut k = m;

    for (idx, &x) in a.iter().enumerate() {
        if x * (n - idx) as u64 <= k {
            k -= x;
        } else {
            println!("{}", k / (n - idx) as u64);
            return;
        }
    }

    println!("infinite");
}
