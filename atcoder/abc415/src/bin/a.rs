use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64;n],
        x: u64,
    }
    println!("{}", if a.contains(&x) { "Yes" } else { "No" });
}
